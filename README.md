# gRPC hello world with TLS

A minimal example demonstrating a gRPC server–client connection using TLS authentication.

## 🛠️ Build Instructions

### Prerequisites

- Rust toolchain (install via [rustup.rs](https://rustup.rs))
- Internet access (to fetch metadata)

### Clone the project

```bash
git clone https://github.com/georgeliao/grpc_hello_world.git
cd grpc_hello_world
```

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --bin helloworld-server
cargo run --bin helloworld-client
```
