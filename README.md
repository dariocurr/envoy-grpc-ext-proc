ca# envoy-grpc-ext-proc

`envoy-grpc-ext-proc` provides client and server stubs for Envoy's
[ext_proc](https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/ext_proc_filter)
protocol, generated and compiled for Rust, using
[Tonic](https://github.com/hyperium/tonic) as the gRPC
implementation. These stubs are sufficient, if used with Tonic, to support a gRPC
client or server that works with the ext_proc protocol.

Builds use git submodules containing protobuf definitions required by ext_proc.
Generated Rust sources are compiled during crate builds.

## Use

```toml
[dependencies]
envoy-grpc-ext-proc = "0.1.3"
```

The build uses `protoc-bin-vendored`; no system `protoc` installation is required.

After cloning, fetch protobuf submodules before building:

```sh
git submodule update --init --recursive
```

Run tests with:

```sh
cargo test
```
