use std::io;

use tonic_build::Builder;

fn main() -> Result<(), io::Error> {
    let b: Builder = tonic_build::configure()
        .build_server(false)
        .build_client(true);
    b.compile(&["rkv/v1/kvstore.proto"], &["proto"])?;
    Ok(())
}
