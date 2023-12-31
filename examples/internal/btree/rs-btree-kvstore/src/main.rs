use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

use tonic::transport::{server::Router, NamedService, Server};

use tonic_health::ServingStatus;

use rs_kvstore::internal::locked::kv_svc_internal_new;

use rs_kvstore::rpc::key_val_service_server::KeyValService;
use rs_kvstore::rpc::key_val_service_server::KeyValServiceServer;

const LISTEN_ADDR_DEFAULT: &str = "127.0.0.1:50051";

fn kvs2name<T>(_: &T) -> &'static str
where
    T: KeyValService,
{
    <KeyValServiceServer<T> as NamedService>::NAME
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let listen_str: String = std::env::var("ENV_LISTEN")
        .ok()
        .unwrap_or_else(|| LISTEN_ADDR_DEFAULT.into());
    let listen: SocketAddr =
        str::parse(listen_str.as_str()).map_err(|e| format!("Invalid addr: {e}"))?;

    let kv_bb = rs_kvstore::internal::bucket::btree::kv_btree_btree_new(); // core KeyValue
    let kv = rs_kvstore::internal::core::kv_new(kv_bb); // kv KeyValue

    let (mut hreport, hsvc) = tonic_health::server::health_reporter();

    let alk: Arc<RwLock<_>> = Arc::new(RwLock::new(kv));
    let kv_svc = kv_svc_internal_new(&alk);
    let kv_name: &str = kvs2name(&kv_svc);
    hreport
        .set_service_status(kv_name, ServingStatus::Serving)
        .await;
    let kv_svr: KeyValServiceServer<_> = KeyValServiceServer::new(kv_svc);

    let mut server = Server::builder();
    let router: Router<_> = server.add_service(hsvc).add_service(kv_svr);

    router
        .serve(listen)
        .await
        .map_err(|e| format!("Unable to listen: {e}"))?;
    Ok(())
}
