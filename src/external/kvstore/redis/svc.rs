use std::time::SystemTime;

use tonic::{Request, Response, Status};

use redis::aio::ConnectionManager;
use redis::{AsyncCommands, Client};

use crate::bucket::checker::Checker;
use crate::cmd::del::DelReq;
use crate::cmd::exists::ExistsReq;
use crate::cmd::get::GetReq;
use crate::cmd::insert::InsertReq;
use crate::cmd::set::SetReq;

use crate::rpc::key_val_service_server::KeyValService;
use crate::rpc::{
    del_response, DelRequest, DelResponse, ExistsRequest, ExistsResponse, GetRequest, GetResponse,
    InsertRequest, InsertResponse, SetRequest, SetResponse, Val,
};

pub struct Svc<C> {
    connection: ConnectionManager,
    checker: C,
}

impl<C> Svc<C> {
    pub async fn get_raw(&self, key: &[u8]) -> Result<Vec<u8>, Status> {
        let mut c: ConnectionManager = self.connection.clone();
        c.get(key)
            .await
            .map_err(|e| Status::internal(format!("Unable to get a val: {e}")))
    }

    pub async fn exists_raw(&self, key: &[u8]) -> Result<bool, Status> {
        let mut c: ConnectionManager = self.connection.clone();
        c.exists(key)
            .await
            .map_err(|e| Status::internal(format!("Unable to check a key: {e}")))
    }

    pub async fn set_raw(&self, key: Vec<u8>, val: Vec<u8>) -> Result<(), Status> {
        let mut c: ConnectionManager = self.connection.clone();
        c.set(key, val)
            .await
            .map_err(|e| Status::internal(format!("Unable to set: {e}")))
    }

    pub async fn del_raw(&self, key: Vec<u8>) -> Result<u64, Status> {
        let mut c: ConnectionManager = self.connection.clone();
        c.del(key)
            .await
            .map_err(|e| Status::internal(format!("Unable to del: {e}")))
    }
}

#[tonic::async_trait]
impl<C> KeyValService for Svc<C>
where
    C: Send + Sync + 'static + Checker,
{
    async fn get(&self, req: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let gr: GetRequest = req.into_inner();
        let checked: GetReq = GetReq::new(gr, &self.checker)?;
        let raw: &[u8] = checked.as_key_bytes();

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
        let checked: SetReq = SetReq::new(sr, &self.checker)?;
        let (key, val) = checked.into_kv();

        self.set_raw(key.k, val.v).await?;
        let set: SystemTime = SystemTime::now();

        let reply: SetResponse = SetResponse {
            set: Some(set).map(|s| s.into()),
        };
        Ok(Response::new(reply))
    }

    async fn del(&self, req: Request<DelRequest>) -> Result<Response<DelResponse>, Status> {
        let sr: DelRequest = req.into_inner();
        let checked: DelReq = DelReq::new(sr, &self.checker)?;
        let key: Vec<u8> = checked.into_key().k;

        let cnt: u64 = self.del_raw(key).await?;
        let reply: DelResponse = match cnt {
            0 => DelResponse {
                status: Some(del_response::Status::Absent(())),
            },
            _ => DelResponse {
                status: Some(SystemTime::now())
                    .map(|s| s.into())
                    .map(del_response::Status::Removed),
            },
        };
        Ok(Response::new(reply))
    }

    /// Warn: this emulates "insert" using "exists" and "set"(TOCTOU)
    ///
    /// 1. Checks if a key is already used or not
    /// 2. Reject the key if it is already used(TOC)
    /// 3. Set the key/val pair if it is not used(TOU)
    async fn insert(
        &self,
        req: Request<InsertRequest>,
    ) -> Result<Response<InsertResponse>, Status> {
        let ir: InsertRequest = req.into_inner();
        let checked: InsertReq = InsertReq::new(ir, &self.checker)?;
        let (key, val) = checked.into_kv();
        let raw: &[u8] = &key.k;
        let found: bool = self.exists_raw(raw).await?;
        match found {
            true => Err(Status::failed_precondition("the key already used")),
            false => {
                self.set_raw(key.k, val.v).await?;

                let reply: InsertResponse = InsertResponse {
                    inserted: Some(SystemTime::now()).map(|s| s.into()),
                };
                Ok(Response::new(reply))
            }
        }
    }

    async fn exists(
        &self,
        req: Request<ExistsRequest>,
    ) -> Result<Response<ExistsResponse>, Status> {
        let er: ExistsRequest = req.into_inner();
        let checked: ExistsReq = ExistsReq::new(er, &self.checker)?;
        let raw: &[u8] = checked.as_key_bytes();

        let found: bool = self.exists_raw(raw).await?;
        let reply: ExistsResponse = ExistsResponse { found };
        Ok(Response::new(reply))
    }
}

pub fn key_val_svc_from_manager<C>(connection: ConnectionManager, checker: C) -> impl KeyValService
where
    C: Send + Sync + 'static + Checker,
{
    Svc {
        connection,
        checker,
    }
}

pub async fn key_val_svc_from_client<C>(c: Client, checker: C) -> Result<impl KeyValService, String>
where
    C: Send + Sync + 'static + Checker,
{
    let connection: ConnectionManager = ConnectionManager::new(c)
        .await
        .map_err(|e| format!("Unable to create a connection manager: {e}"))?;
    Ok(key_val_svc_from_manager(connection, checker))
}

pub async fn key_val_svc_new<C>(conn_str: &str, checker: C) -> Result<impl KeyValService, String>
where
    C: Send + Sync + 'static + Checker,
{
    let client: Client = Client::open(conn_str).map_err(|e| format!("Unable to open: {e}"))?;
    key_val_svc_from_client(client, checker).await
}
