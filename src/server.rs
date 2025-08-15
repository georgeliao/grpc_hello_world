use hello::greeter_server::{Greeter, GreeterServer};
use hello::{HelloReply, HelloRequest};
use tokio::fs;
use tonic::transport::{Certificate, Identity, ServerTlsConfig};
use tonic::{Request, Response, Status, transport::Server};

pub mod hello {
    tonic::include_proto!("hello");
}

// ✅ Implement the Greeter trait for our service
#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let name = request.into_inner().name;
        let reply = HelloReply {
            message: format!("Hello, {}!", name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- required for TLS ---
    let server_cert = fs::read("../..//tls/server.pem").await?;
    let server_key = fs::read("../../tls/server_key.pem").await?;
    let identity = Identity::from_pem(server_cert, server_key);

    let addr = "[::1]:50051".parse()?;
    println!("GreeterServer (TLS) listening on {}", addr);

    Server::builder()
        .tls_config(ServerTlsConfig::new().identity(identity))?
        .add_service(GreeterServer::new(MyGreeter::default()))
        .serve(addr)
        .await?;

    Ok(())
}
