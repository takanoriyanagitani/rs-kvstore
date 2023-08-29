use std::env;
use std::net::SocketAddr;

use tonic::transport::{server::Router, Server};

use rs_kvstore::rpc::key_val_service_server::KeyValServiceServer;

const DEFAULT_LISTEN: &str = "127.0.0.1:50051";
const DEFAULT_REDIS: &str = "redis://127.0.0.1";

#[tokio::main]
async fn main() -> Result<(), String> {
    let redis_con_str: Option<String> = env::var("ENV_REDIS").ok();
    let svc = rs_kvstore::external::kvstore::redis::svc::key_val_svc_new(
        redis_con_str.as_deref().unwrap_or(DEFAULT_REDIS),
    )
    .await?;
    let svr: KeyValServiceServer<_> = KeyValServiceServer::new(svc);

    let mut server: Server = Server::builder();
    let router: Router<_> = server.add_service(svr);

    let addr: Option<String> = env::var("ENV_ADDR").ok();
    let a: SocketAddr = str::parse(addr.as_deref().unwrap_or(DEFAULT_LISTEN))
        .map_err(|e| format!("Invalid addr: {e}"))?;

    router
        .serve(a)
        .await
        .map_err(|e| format!("Unable to listen: {e}"))?;
    Ok(())
}
