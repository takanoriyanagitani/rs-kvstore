use tonic::Status;

use crate::bucket::bkt::Bucket;
use crate::bucket::checker::Checker;
use crate::uuid::Uuid;

use crate::rpc::GetRequest;
use crate::rpc::Key;

pub struct GetReq {
    request_id: Uuid,
    bucket: Bucket,
    key: Key,
}

impl GetReq {
    pub fn new<C>(g: GetRequest, checker: &C) -> Result<Self, Status>
    where
        C: Checker,
    {
        let request_id: Uuid = g
            .request_id
            .map(|u| (&u).into())
            .ok_or_else(|| Status::invalid_argument("request id missing"))?;
        let bucket: Bucket = g
            .bucket
            .ok_or_else(|| {
                Status::invalid_argument(format!("bucket missing. request id: {request_id}"))
            })
            .and_then(|b| Bucket::new(b.b, checker))
            .map_err(|e| {
                Status::invalid_argument(format!("invalid bucket(request id={request_id}): {e}"))
            })?;
        let key: Key = g.key.ok_or_else(|| {
            Status::invalid_argument(format!("key missing. request id: {request_id}"))
        })?;
        Ok(Self {
            request_id,
            bucket,
            key,
        })
    }

    pub fn as_request(&self) -> Uuid {
        self.request_id
    }

    pub fn as_key(&self) -> &Key {
        &self.key
    }
    pub fn as_key_bytes(&self) -> &[u8] {
        &self.as_key().k
    }

    pub fn as_bucket(&self) -> &Bucket {
        &self.bucket
    }
}
