# envoy-ext-proc-stubs

This module contains the client and server stubs for Envoy's
[ext_proc](https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/ext_proc_filter)
protocol, generated and compiled for Rust, using
[Tonic](https://github.com/hyperium/tonic) as the gRPC
implementation. These stubs are sufficient, if used with Tonic, to support a gRPC
client or server that works with the ext_proc protocol.

The stubs build because they use git submodules in order to integrate with
the four different GitHub repositories that house all the protobuf files
needed to support the ext_proc protocol.

In order to build this crate, you'll need "protoc" installed on your machine.
[See the instructions](https://github.com/protocolbuffers/protobuf#protocol-compiler-installation).
