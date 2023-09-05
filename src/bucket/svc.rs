use tonic::{Request, Response, Status};

use crate::uuid::Uuid;

use crate::bucket::bkt::Bucket;
use crate::bucket::checker::Checker;
use crate::bucket::prefix::BucketAsPrefix;

use crate::cmd::del::DelReq;
use crate::cmd::exists::ExistsReq;
use crate::cmd::get::GetReq;
use crate::cmd::insert::InsertReq;
use crate::cmd::set::SetReq;
use crate::cmd::truncate::TruncateReq;

use crate::rpc::key_val_service_server::KeyValService;
use crate::rpc::{
    DelRequest, DelResponse, ExistsRequest, ExistsResponse, GetRequest, GetResponse, InsertRequest,
    InsertResponse, Key, SetRequest, SetResponse, TruncateRequest, TruncateResponse,
};

pub struct SimpleKvSvc<P, K, C> {
    virtual_bucket: P,
    simple: K,
    checker: C,
}

#[tonic::async_trait]
impl<P, K, C> KeyValService for SimpleKvSvc<P, K, C>
where
    P: Send + Sync + 'static + BucketAsPrefix,
    K: Send + Sync + 'static + KeyValService,
    C: Send + Sync + 'static + Checker,
{
    async fn get(&self, req: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let gr: GetRequest = req.into_inner();
        let checked: GetReq = GetReq::new(gr, &self.checker)?;

        let reqid: Uuid = checked.as_request();

        let bkt: &Bucket = checked.as_bucket();
        let key: &Key = checked.as_key();
        let neo: Key = self.virtual_bucket.key_with_bucket(bkt, key)?;

        let sreq: GetRequest = GetRequest {
            request_id: Some(reqid).map(|u| u.into()),
            bucket: Some(Bucket::default()).map(|b| b.into()),
            key: Some(neo),
        };
        self.simple.get(Request::new(sreq)).await
    }

    async fn set(&self, req: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        let sr: SetRequest = req.into_inner();
        let checked: SetReq = SetReq::new(sr, &self.checker)?;

        let reqid: Uuid = checked.as_request();
        let bkt: &Bucket = checked.as_bucket();
        let key: &Key = checked.as_key();
        let neo: Key = self.virtual_bucket.key_with_bucket(bkt, key)?;
        let (_, val) = checked.into_kv();

        let sreq: SetRequest = SetRequest {
            request_id: Some(reqid).map(|u| u.into()),
            bucket: Some(Bucket::default()).map(|b| b.into()),
            key: Some(neo),
            val: Some(val),
        };
        self.simple.set(Request::new(sreq)).await
    }

    async fn del(&self, req: Request<DelRequest>) -> Result<Response<DelResponse>, Status> {
        let sr: DelRequest = req.into_inner();
        let checked: DelReq = DelReq::new(sr, &self.checker)?;

        let reqid: Uuid = checked.as_request();
        let bkt: &Bucket = checked.as_bucket();
        let key: &Key = checked.as_key();
        let neo: Key = self.virtual_bucket.key_with_bucket(bkt, key)?;

        let sreq: DelRequest = DelRequest {
            request_id: Some(reqid).map(|u| u.into()),
            bucket: Some(Bucket::default()).map(|b| b.into()),
            key: Some(neo),
        };
        self.simple.del(Request::new(sreq)).await
    }

    async fn truncate(
        &self,
        req: Request<TruncateRequest>,
    ) -> Result<Response<TruncateResponse>, Status> {
        let sr: TruncateRequest = req.into_inner();
        let checked: TruncateReq = TruncateReq::new(sr, &self.checker)?;

        let reqid: Uuid = checked.as_request();

        let sreq: TruncateRequest = TruncateRequest {
            request_id: Some(reqid).map(|u| u.into()),
            bucket: Some(Bucket::default()).map(|b| b.into()),
        };
        self.simple.truncate(Request::new(sreq)).await
    }

    async fn insert(
        &self,
        req: Request<InsertRequest>,
    ) -> Result<Response<InsertResponse>, Status> {
        let ir: InsertRequest = req.into_inner();
        let checked: InsertReq = InsertReq::new(ir, &self.checker)?;

        let reqid: Uuid = checked.as_request();
        let bkt: &Bucket = checked.as_bucket();
        let key: &Key = checked.as_key();
        let neo: Key = self.virtual_bucket.key_with_bucket(bkt, key)?;
        let (_, val) = checked.into_kv();

        let sreq: InsertRequest = InsertRequest {
            request_id: Some(reqid).map(|u| u.into()),
            bucket: Some(Bucket::default()).map(|b| b.into()),
            key: Some(neo),
            val: Some(val),
        };
        self.simple.insert(Request::new(sreq)).await
    }

    async fn exists(
        &self,
        req: Request<ExistsRequest>,
    ) -> Result<Response<ExistsResponse>, Status> {
        let er: ExistsRequest = req.into_inner();
        let checked: ExistsReq = ExistsReq::new(er, &self.checker)?;

        let reqid: Uuid = checked.as_request();

        let bkt: &Bucket = checked.as_bucket();
        let key: &Key = checked.as_key();
        let neo: Key = self.virtual_bucket.key_with_bucket(bkt, key)?;

        let sreq: ExistsRequest = ExistsRequest {
            request_id: Some(reqid).map(|u| u.into()),
            bucket: Some(Bucket::default()).map(|b| b.into()),
            key: Some(neo),
        };
        self.simple.exists(Request::new(sreq)).await
    }
}

pub fn bucket_as_prefix_svc_new<P, K, C>(
    virtual_bucket: P,
    original: K,
    checker: C,
) -> impl KeyValService
where
    P: Send + Sync + 'static + BucketAsPrefix,
    K: Send + Sync + 'static + KeyValService,
    C: Send + Sync + 'static + Checker,
{
    SimpleKvSvc {
        virtual_bucket,
        simple: original,
        checker,
    }
}
