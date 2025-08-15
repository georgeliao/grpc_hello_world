fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure().compile_protos(&["proto/hello.proto"], &["../src/rpc"])?;
    Ok(())
}
