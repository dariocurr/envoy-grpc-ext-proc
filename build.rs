use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    unsafe {
        env::set_var("PROTOC", protoc);
    }
    tonic_prost_build::configure()
        .build_client(true)
        .build_server(true)
        .compile_protos(
            &["data-plane-api/envoy/service/ext_proc/v3/external_processor.proto"],
            &["data-plane-api", "udpa", "protoc-gen-validate", "xds"],
        )?;
    Ok(())
}
