# webrpc rust

- Client: Rust CLI
- Server: Node JS CLI

Example of generating a webrpc server and client from [service.ridl](./service.ridl) schema.

Rust server code isn't done yet :)

## Usage

1. Install rust
2. Install pnpm
3. $ `make bootstrap`
4. $ `make run-server`
5. While the server is running, $ `make run-client`

## Notes

The cool thing about webrpc and other schema-driven rpc libraries (like grpc), is that you can
generate a Go client for this node server just by running:

`webrpc-gen -schema=service.ridl -target=go -pkg=proto -client -out=./proto/client.gen.go`

and tada, your Go programs now have full type definitions and network communication to the node
server!
