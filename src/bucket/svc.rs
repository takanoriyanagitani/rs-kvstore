use tonic::{Request, Response, Status};

use crate::uuid::Uuid;

use crate::bucket::bkt::Bucket;
use crate::bucket::checker::Checker;
use crate::bucket::prefix::BucketAsPrefix;

use crate::cmd::get::GetReq;
use crate::cmd::set::SetReq;

use crate::rpc::key_val_service_server::KeyValService;
use crate::rpc::{GetRequest, GetResponse, Key, SetRequest, SetResponse};

struct SimpleKvSvc<P, K, C> {
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