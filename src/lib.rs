#![deny(warnings)]

//! # Envoy External Processor gRPC Stubs
//!
//! Pre-compiled gRPC client and server stubs for Envoy's external processor (`ext_proc`) protocol.
//! Includes all necessary protobuf definitions and generated Rust code for building external
//! processors that intercept and modify HTTP requests and responses in Envoy.
//!
//! ## Server Example
//!
//! Here's a complete example of implementing an external processor server:
//!
//! ```rust
//! use std::pin;
//! use futures::Stream;
//! use envoy_grpc_ext_proc::envoy::service::ext_proc::v3;
//! use tonic::{Request, Response, Status, Streaming, async_trait, transport,};
//!
//! struct MyExternalProcessor;
//!
//! #[async_trait]
//! impl v3::external_processor_server::ExternalProcessor for MyExternalProcessor {
//!     type ProcessStream = pin::Pin<
//!         Box<dyn Stream<Item = Result<v3::ProcessingResponse, Status>> + Send + 'static>,
//!     >;
//!
//!     async fn process(
//!         &self,
//!         request: Request<Streaming<v3::ProcessingRequest>>,
//!     ) -> Result<Response<Self::ProcessStream>, Status> {
//!         todo!()
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let addr = "[::1]:50051".parse()?;
//!     let processor = MyExternalProcessor;
//!
//!     transport::Server::builder()
//!         .add_service(v3::external_processor_server::ExternalProcessorServer::new(processor))
//!         .serve(addr);
//!     //  .await?;  // to avoid server running on doc tests
//!
//!     Ok(())
//! }
//! ```
//!

/// Envoy API definitions and services
pub mod envoy {
    /// Configuration-related types and structures
    pub mod config {
        /// Core configuration types used throughout Envoy
        pub mod core {
            /// Version 3 of the core configuration API
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.config.core.v3.rs"));
            }
        }
    }
    /// Envoy extensions and filters
    pub mod extensions {
        /// HTTP filters for request/response processing
        pub mod filters {
            /// HTTP-specific filters
            pub mod http {
                /// External processor HTTP filter
                pub mod ext_proc {
                    /// Version 3 of the ext_proc filter API
                    pub mod v3 {
                        include!(concat!(
                            env!("OUT_DIR"),
                            "/envoy.extensions.filters.http.ext_proc.v3.rs"
                        ));
                    }
                }
            }
        }
    }
    /// Common type definitions used across Envoy
    pub mod r#type {
        /// Version 3 of the type definitions
        pub mod v3 {
            include!(concat!(env!("OUT_DIR"), "/envoy.r#type.v3.rs"));
        }
    }
    /// Envoy service definitions
    pub mod service {
        /// External processor service
        pub mod ext_proc {
            /// Version 3 of the external processor service API
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.service.ext_proc.v3.rs"));
            }
        }
    }
}

/// Protocol buffer validation rules and constraints
pub mod validate {
    include!(concat!(env!("OUT_DIR"), "/validate.rs"));
}

/// Universal Data Plane API (UDPA) definitions
pub mod udpa {
    /// UDPA annotations for metadata and configuration
    pub mod annotations {
        include!(concat!(env!("OUT_DIR"), "/udpa.annotations.rs"));
    }
}

/// xDS (configuration discovery service) API definitions
pub mod xds {
    /// xDS annotations for configuration metadata
    pub mod annotations {
        /// Version 3 of xDS annotations
        pub mod v3 {
            include!(concat!(env!("OUT_DIR"), "/xds.annotations.v3.rs"));
        }
    }
    /// Core xDS types and structures
    pub mod core {
        /// Version 3 of xDS core API
        pub mod v3 {
            include!(concat!(env!("OUT_DIR"), "/xds.core.v3.rs"));
        }
    }
}
