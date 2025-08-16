use hello::HelloRequest;
use hello::greeter_client::GreeterClient;
use std::path::Path;
use tokio::fs;
use tonic::Request;
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Endpoint, Identity};

pub mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tls_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tls");
    // Trust anchor for the **server** (CA that signed localhost.pem).
    let server_ca_pem = fs::read(tls_dir.join("root_cert.pem")).await?; // replace if needed
    // Client identity (cert+key) signed by the CA the server trusts.
    let client_cert_pem = fs::read(tls_dir.join("client_cert.pem")).await?;
    let client_key_pem = fs::read(tls_dir.join("client_key.pem")).await?;

    let tls = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(server_ca_pem))
        .identity(Identity::from_pem(client_cert_pem, client_key_pem))
        .domain_name("localhost");

    let channel: Channel = Endpoint::from_shared("https://[::1]:50051".to_string())?
        .tls_config(tls)?
        .connect()
        .await?;

    let mut client = GreeterClient::new(channel);
    let resp = client
        .say_hello(Request::new(HelloRequest {
            name: "Rustacean".into(),
        }))
        .await?;
    println!("Response = {:?}", resp.into_inner().message);

    Ok(())
}
