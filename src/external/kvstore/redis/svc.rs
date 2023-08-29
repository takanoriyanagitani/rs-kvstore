use std::time::SystemTime;

use tonic::{Request, Response, Status};

use redis::aio::ConnectionManager;
use redis::{AsyncCommands, Client};

use crate::uuid::Uuid;

use crate::rpc::key_val_service_server::KeyValService;
use crate::rpc::{GetRequest, GetResponse, Key, SetRequest, SetResponse, Val};

pub struct Svc {
    connection: ConnectionManager,
}

impl Svc {
    pub async fn get_raw(&self, key: &[u8]) -> Result<Vec<u8>, Status> {
        let mut c: ConnectionManager = self.connection.clone();
        c.get(key)
            .await
            .map_err(|e| Status::internal(format!("Unable to get a val: {e}")))
    }

    pub async fn set_raw(&self, key: Vec<u8>, val: Vec<u8>) -> Result<(), Status> {
        let mut c: ConnectionManager = self.connection.clone();
        c.set(key, val)
            .await
            .map_err(|e| Status::internal(format!("Unable to set: {e}")))
    }
}

#[tonic::async_trait]
impl KeyValService for Svc {
    async fn get(&self, req: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let gr: GetRequest = req.into_inner();
        let request_id: Uuid = gr
            .request_id
            .as_ref()
            .map(Uuid::from)
            .ok_or_else(|| Status::invalid_argument("request id missing"))?;
        let key: Key = gr.key.ok_or_else(|| {
            Status::invalid_argument(format!("key missing. request id: {request_id}"))
        })?;
        let raw: &[u8] = &key.k;

        let val: Vec<u8> = self.get_raw(raw).await?;
        let got: SystemTime = SystemTime::now();

        let v: Val = Val { v: val };
        let reply: GetResponse = GetResponse {
            val: Some(v),
            got: Some(got).map(|s| s.into()),
        };
        Ok(Response::new(reply))
    }

    async fn set(&self, req: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        let sr: SetRequest = req.into_inner();
        let request_id: Uuid = sr
            .request_id
            .as_ref()
            .map(Uuid::from)
            .ok_or_else(|| Status::invalid_argument("request id missing"))?;
        let key: Key = sr.key.ok_or_else(|| {
            Status::invalid_argument(format!("key missing. request id: {request_id}"))
        })?;
        let val: Val = sr.val.ok_or_else(|| {
            Status::invalid_argument(format!("val missing. request id: {request_id}"))
        })?;

        self.set_raw(key.k, val.v).await?;
        let set: SystemTime = SystemTime::now();

        let reply: SetResponse = SetResponse {
            set: Some(set).map(|s| s.into()),
        };
        Ok(Response::new(reply))
    }
}

pub fn key_val_svc_from_manager(connection: ConnectionManager) -> impl KeyValService {
    Svc { connection }
}

pub async fn key_val_svc_from_client(c: Client) -> Result<impl KeyValService, String> {
    let connection: ConnectionManager = ConnectionManager::new(c)
        .await
        .map_err(|e| format!("Unable to create a connection manager: {e}"))?;
    Ok(key_val_svc_from_manager(connection))
}

pub async fn key_val_svc_new(conn_str: &str) -> Result<impl KeyValService, String> {
    let client: Client = Client::open(conn_str).map_err(|e| format!("Unable to open: {e}"))?;
    key_val_svc_from_client(client).await
}
