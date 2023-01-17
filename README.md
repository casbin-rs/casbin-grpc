# casbin-grpc
Casbin-gRPC provides gRPC interface for Casbin authorization which is implemented with Rust.

## How to use?

### Prerequisites
Protobuf (if not installed):
As Casbin-Server uses gRPC, you need to install [Protocol Buffers](https://github.com/golang/protobuf#installation) first to generate the `.proto file`, it will auto execute by [build.rs](./build.rs).

### Start server
```
cargo run --bin casbin-server
```