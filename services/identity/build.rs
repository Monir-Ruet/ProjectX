use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("../shared/protos");

    tonic_prost_build::configure()
        .build_server(true)
        .compile_protos(
            &[proto_root.join("user.proto"), proto_root.join("auth.proto")],
            &[proto_root],
        )?;

    Ok(())
}
