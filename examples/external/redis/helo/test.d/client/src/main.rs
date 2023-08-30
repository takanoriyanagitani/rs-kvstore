use std::env;

use tokio::runtime::{Builder, Runtime};

use tonic::transport::Channel;
use tonic::{Response, Status};

pub mod kvsvc {
    tonic::include_proto!("rkv.v1");
}

use kvsvc::key_val_service_client::KeyValServiceClient;
use kvsvc::{GetRequest, GetResponse, SetRequest, SetResponse};
use kvsvc::{Key, Uuid, Val};

const ADDR_DEFAULT: &str = "http://127.0.0.1:50051";

struct KeyValClient {
    client: KeyValServiceClient<Channel>,
    runtime: Runtime,
}

impl KeyValClient {
    fn new(addr: &str, runtime: Runtime) -> Result<Self, String> {
        let fclient = KeyValServiceClient::connect(addr.to_string());
        let client: KeyValServiceClient<_> = runtime
            .block_on(fclient)
            .map_err(|e| format!("Unable to connect: {e}"))?;
        Ok(Self { client, runtime })
    }

    fn set(&mut self, req: SetRequest) -> Result<Response<SetResponse>, Status> {
        self.runtime.block_on(self.client.set(req))
    }

    fn get(&mut self, req: GetRequest) -> Result<Response<GetResponse>, Status> {
        self.runtime.block_on(self.client.get(req))
    }
}

fn main() -> Result<(), String> {
    let addr: String = env::var("ENV_ADDR")
        .ok()
        .unwrap_or_else(|| ADDR_DEFAULT.into());

    let runtime: Runtime = Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| format!("Unable to build a runtime: {e}"))?;

    let mut client: KeyValClient = KeyValClient::new(addr.as_str(), runtime)?;

    client
        .set(SetRequest {
            request_id: Some(Uuid {
                hi: 0x20230830,
                lo: 0x103513,
            }),
            bucket: None,
            key: Some(Key {
                k: (*b"helo").into(),
            }),
            val: Some(Val {
                v: vec![37, 76],
            }),
        })
        .map_err(|e| format!("Unable to set: {e}"))?;

    let res: Response<_> = client
        .get(GetRequest {
            request_id: Some(Uuid {
                hi: 0x20230830,
                lo: 0x103513,
            }),
            bucket: None,
            key: Some(Key {
                k: (*b"helo").into(),
            }),
        })
        .map_err(|e| format!("Unable to set: {e}"))?;

    let gr: GetResponse = res.into_inner();
    println!("got: {gr:#?}");
    Ok(())
}
