fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_client(true)
        .build_server(true)
        .compile_protos(
            &["data-plane-api/envoy/service/ext_proc/v3/external_processor.proto"],
            &["data-plane-api", "udpa", "protoc-gen-validate", "xds"],
        )?;
    Ok(())
}
