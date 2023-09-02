use std::sync::{Arc, RwLock};

use tonic::{Request, Response, Status};

use crate::rpc::key_val_service_server::KeyValService;
use crate::rpc::{
    ExistsRequest, ExistsResponse, GetRequest, GetResponse, InsertRequest, InsertResponse,
    SetRequest, SetResponse,
};

use crate::internal::kv::KeyValue;

pub struct Locked<K> {
    kvstore: Arc<RwLock<K>>,
}

impl<K> Locked<K> {
    pub fn read<F, T>(&self, f: F) -> Result<T, Status>
    where
        F: FnOnce(&K) -> Result<T, Status>,
    {
        match self.kvstore.read() {
            Err(e) => Err(Status::internal(format!("UNABLE TO READ LOCK: {e}"))),
            Ok(guard) => {
                let mk: &K = &guard;
                f(mk)
            }
        }
    }

    pub fn write<F, T>(&self, f: F) -> Result<T, Status>
    where
        F: FnOnce(&mut K) -> Result<T, Status>,
    {
        match self.kvstore.write() {
            Err(e) => Err(Status::internal(format!("UNABLE TO WRITE LOCK: {e}"))),
            Ok(mut guard) => {
                let mk: &mut K = &mut guard;
                f(mk)
            }
        }
    }
}

impl<K> Locked<K>
where
    K: KeyValue,
{
    pub fn get(&self, req: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        self.read(|mk: &K| mk.get(req))
    }

    pub fn exists(&self, req: Request<ExistsRequest>) -> Result<Response<ExistsResponse>, Status> {
        self.read(|mk: &K| mk.exists(req))
    }

    pub fn set(&self, req: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        self.write(|mk: &mut K| mk.set(req))
    }

    pub fn insert(&self, req: Request<InsertRequest>) -> Result<Response<InsertResponse>, Status> {
        self.write(|mk: &mut K| mk.insert(req))
    }
}

#[tonic::async_trait]
impl<K> KeyValService for Locked<K>
where
    K: Send + Sync + 'static + KeyValue,
{
    async fn get(&self, req: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        self.get(req)
    }

    async fn set(&self, req: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        self.set(req)
    }

    async fn insert(
        &self,
        req: Request<InsertRequest>,
    ) -> Result<Response<InsertResponse>, Status> {
        self.insert(req)
    }

    async fn exists(
        &self,
        req: Request<ExistsRequest>,
    ) -> Result<Response<ExistsResponse>, Status> {
        self.exists(req)
    }
}

pub fn kv_svc_internal_new<K>(internal: &Arc<RwLock<K>>) -> impl KeyValService
where
    K: Send + Sync + 'static + KeyValue,
{
    Locked {
        kvstore: internal.clone(),
    }
}
