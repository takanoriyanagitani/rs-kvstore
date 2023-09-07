use std::collections::BTreeMap;
use std::time::SystemTime;

use tonic::{Request, Response, Status};

use crate::bucket::bkt::Bucket;

use crate::internal::kv;

use crate::cmd::del::DelReq;
use crate::cmd::exists::ExistsReq;
use crate::cmd::get::GetReq;
use crate::cmd::insert::InsertReq;
use crate::cmd::set::SetReq;
use crate::cmd::truncate::TruncateReq;

use crate::rpc::{
    del_response, truncate_response, DelRequest, DelResponse, ExistsRequest, ExistsResponse,
    GetRequest, GetResponse, InsertRequest, InsertResponse, Key, SetRequest, SetResponse,
    TruncateRequest, TruncateResponse, Val,
};

pub trait KeyValue {
    fn get(&self, b: &Bucket, k: &Key) -> Option<Val>;
    fn exists(&self, b: &Bucket) -> bool;
    fn exists_key(&self, b: &Bucket, k: &Key) -> bool;

    fn set(&mut self, b: Bucket, k: Key, v: Val) -> Option<Val>;

    fn del(&mut self, b: &Bucket, k: &Key) -> Option<Val>;
    fn truncate(&mut self, b: &Bucket);

    fn ins(&mut self, b: Bucket, k: Key, v: Val) -> Result<(), String> {
        match self.set(b.clone(), k.clone(), v) {
            None => Ok(()),
            Some(old) => {
                self.set(b, k, old);
                Err(String::from("the key already used"))
            }
        }
    }
}

impl KeyValue for BTreeMap<Vec<u8>, Val> {
    fn get(&self, _: &Bucket, k: &Key) -> Option<Val> {
        self.get(&k.k).cloned()
    }

    fn exists(&self, _: &Bucket) -> bool {
        true
    }

    fn exists_key(&self, _: &Bucket, k: &Key) -> bool {
        self.contains_key(&k.k)
    }

    fn set(&mut self, _: Bucket, k: Key, v: Val) -> Option<Val> {
        self.insert(k.k, v)
    }

    fn del(&mut self, _: &Bucket, k: &Key) -> Option<Val> {
        self.remove(&k.k)
    }

    fn truncate(&mut self, _: &Bucket) {
        self.clear()
    }
}

impl<K> kv::KeyValue for K
where
    K: KeyValue,
{
    fn get(&self, req: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let gr: GetRequest = req.into_inner();
        let nocheck: GetReq = gr.try_into()?;

        let b: &Bucket = nocheck.as_bucket();
        let k: &Key = nocheck.as_key();
        let v: Val = self.get(b, k).ok_or_else(|| {
            Status::not_found(format!(
                "No val found for key: {:#?}",
                nocheck.as_key_bytes()
            ))
        })?;
        let reply: GetResponse = GetResponse {
            val: Some(v),
            got: Some(SystemTime::now()).map(|s| s.into()),
        };
        Ok(Response::new(reply))
    }
    fn exists(&self, req: Request<ExistsRequest>) -> Result<Response<ExistsResponse>, Status> {
        let er: ExistsRequest = req.into_inner();
        let nocheck: ExistsReq = er.try_into()?;
        let b: &Bucket = nocheck.as_bucket();
        let k: &Key = nocheck.as_key();
        let found: bool = self.exists_key(b, k);
        let reply: ExistsResponse = ExistsResponse { found };
        Ok(Response::new(reply))
    }

    fn set(&mut self, req: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        let sr: SetRequest = req.into_inner();
        let nochecked: SetReq = sr.try_into()?;
        let (b, k, v) = nochecked.into_unpacked();
        self.set(b, k, v);
        let reply: SetResponse = SetResponse {
            set: Some(SystemTime::now()).map(|s| s.into()),
        };
        Ok(Response::new(reply))
    }
    fn del(&mut self, req: Request<DelRequest>) -> Result<Response<DelResponse>, Status> {
        let sr: DelRequest = req.into_inner();
        let nochecked: DelReq = sr.try_into()?;
        let b: &Bucket = nochecked.as_bucket();
        let k: &Key = nochecked.as_key();
        let reply: DelResponse = match self.del(b, k) {
            None => DelResponse {
                status: Some(del_response::Status::Absent(())),
            },
            Some(_) => DelResponse {
                status: Some(SystemTime::now())
                    .map(|st| st.into())
                    .map(del_response::Status::Removed),
            },
        };
        Ok(Response::new(reply))
    }
    fn truncate(
        &mut self,
        req: Request<TruncateRequest>,
    ) -> Result<Response<TruncateResponse>, Status> {
        let sr: TruncateRequest = req.into_inner();
        let nocheck: TruncateReq = sr.try_into()?;
        let b: &Bucket = nocheck.as_bucket();
        let found: bool = self.exists(b);
        let empty: bool = !found;
        self.truncate(b);
        let reply: TruncateResponse = match empty {
            true => TruncateResponse {
                status: Some(truncate_response::Status::Absent(())),
            },
            false => TruncateResponse {
                status: Some(SystemTime::now())
                    .map(|st| st.into())
                    .map(truncate_response::Status::Truncated),
            },
        };
        Ok(Response::new(reply))
    }
    fn insert(&mut self, req: Request<InsertRequest>) -> Result<Response<InsertResponse>, Status> {
        let ir: InsertRequest = req.into_inner();
        let nocheck: InsertReq = ir.try_into()?;
        let (b, k, v) = nocheck.into_unpacked();
        self.ins(b, k, v).map_err(Status::failed_precondition)?;
        let reply = InsertResponse {
            inserted: Some(SystemTime::now()).map(|s| s.into()),
        };
        Ok(Response::new(reply))
    }
}

pub fn kv_new<K>(core_kv: K) -> impl kv::KeyValue
where
    K: KeyValue,
{
    core_kv
}
