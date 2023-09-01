use std::collections::BTreeMap;
use std::time::SystemTime;

use tonic::{Request, Response, Status};

use crate::bucket::checker::Checker;

use crate::cmd::exists::ExistsReq;
use crate::cmd::get::GetReq;
use crate::cmd::set::SetReq;

use crate::internal::kv::KeyValue;

use crate::rpc::{
    ExistsRequest, ExistsResponse, GetRequest, GetResponse, SetRequest, SetResponse, Val,
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
