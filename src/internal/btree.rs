use std::collections::BTreeMap;
use std::time::SystemTime;

use tonic::{Request, Response, Status};

use crate::bucket::checker::Checker;

use crate::cmd::del::DelReq;
use crate::cmd::drop::DropReq;
use crate::cmd::exists::ExistsReq;
use crate::cmd::get::GetReq;
use crate::cmd::insert::InsertReq;
use crate::cmd::set::SetReq;
use crate::cmd::truncate::TruncateReq;

use crate::internal::kv::KeyValue;

use crate::rpc::{
    del_response, drop_response, truncate_response, DelRequest, DelResponse, DropRequest,
    DropResponse, ExistsRequest, ExistsResponse, GetRequest, GetResponse, InsertRequest,
    InsertResponse, SetRequest, SetResponse, TruncateRequest, TruncateResponse, Val,
};

pub struct BTree<C> {
    checker: C,
    m: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl<C> KeyValue for BTree<C>
where
    C: Checker,
{
    fn get(&self, req: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let gr: GetRequest = req.into_inner();
        let checked: GetReq = GetReq::new(gr, &self.checker)?;
        let key_bytes: &[u8] = checked.as_key_bytes();
        let val: &[u8] = self
            .m
            .get(key_bytes)
            .ok_or_else(|| Status::not_found(format!("No val found for key: {:#?}", key_bytes)))?;
        let got: SystemTime = SystemTime::now();
        let v: Val = Val { v: val.into() };
        let reply: GetResponse = GetResponse {
            val: Some(v),
            got: Some(got).map(|s| s.into()),
        };
        Ok(Response::new(reply))
    }

    fn exists(&self, req: Request<ExistsRequest>) -> Result<Response<ExistsResponse>, Status> {
        let er: ExistsRequest = req.into_inner();
        let checked: ExistsReq = ExistsReq::new(er, &self.checker)?;
        let key_bytes: &[u8] = checked.as_key_bytes();
        let found: bool = self.m.contains_key(key_bytes);
        let reply: ExistsResponse = ExistsResponse { found };
        Ok(Response::new(reply))
    }

    fn set(&mut self, req: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        let sr: SetRequest = req.into_inner();
        let checked: SetReq = SetReq::new(sr, &self.checker)?;
        let (key, val) = checked.into_kv();
        let k: Vec<u8> = key.k;
        let v: Vec<u8> = val.v;
        self.m.insert(k, v);
        let reply: SetResponse = SetResponse {
            set: Some(SystemTime::now()).map(|s| s.into()),
        };
        Ok(Response::new(reply))
    }

    fn del(&mut self, req: Request<DelRequest>) -> Result<Response<DelResponse>, Status> {
        let sr: DelRequest = req.into_inner();
        let checked: DelReq = DelReq::new(sr, &self.checker)?;
        let key = checked.into_key();
        let k: Vec<u8> = key.k;
        let reply: DelResponse = match self.m.remove(&k) {
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
        TruncateReq::new(sr, &self.checker)?;
        let empty: bool = self.m.is_empty();
        self.m.clear();
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

    fn drop(&mut self, req: Request<DropRequest>) -> Result<Response<DropResponse>, Status> {
        let sr: DropRequest = req.into_inner();
        DropReq::new(sr, &self.checker)?;
        let empty: bool = self.m.is_empty();
        self.m.clear();
        let reply: DropResponse = match empty {
            true => DropResponse {
                status: Some(drop_response::Status::Absent(())),
            },
            false => DropResponse {
                status: Some(SystemTime::now())
                    .map(|st| st.into())
                    .map(drop_response::Status::Dropped),
            },
        };
        Ok(Response::new(reply))
    }

    fn insert(&mut self, req: Request<InsertRequest>) -> Result<Response<InsertResponse>, Status> {
        let ir: InsertRequest = req.into_inner();
        let checked: InsertReq = InsertReq::new(ir, &self.checker)?;
        let (key, val) = checked.into_kv();
        let k: Vec<u8> = key.k;
        let v: Vec<u8> = val.v;
        match self.m.insert(k.clone(), v) {
            None => Ok(InsertResponse {
                inserted: Some(SystemTime::now()).map(|s| s.into()),
            })
            .map(Response::new),
            Some(overwritten) => {
                self.m.insert(k, overwritten);
                Err(Status::failed_precondition("the key already used"))
            }
        }
    }
}

pub fn kvstore_btree_new<C>(checker: C) -> impl KeyValue
where
    C: Checker,
{
    BTree {
        checker,
        m: BTreeMap::default(),
    }
}
