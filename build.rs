fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().build_server(false).compile(
        &["proto/joseluis8906/protobuf/delivery/storemanager.proto"],
        &["proto/joseluis8906", "proto/protocolbuffers"],
    )?;

    Ok(())
}
