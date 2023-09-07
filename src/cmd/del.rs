use tonic::Status;

use crate::bucket::bkt::Bucket;
use crate::bucket::checker::{Checker, NoChecker};
use crate::uuid::Uuid;

use crate::rpc::{DelRequest, Key};

pub struct DelReq {
    request_id: Uuid,
    bucket: Bucket,
    key: Key,
}

impl DelReq {
    pub fn new<C>(g: DelRequest, checker: &C) -> Result<Self, Status>
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

    pub fn into_key(self) -> Key {
        self.key
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

    pub fn into_unpacked(self) -> (Bucket, Key) {
        (self.bucket, self.key)
    }
}

impl TryFrom<DelRequest> for DelReq {
    type Error = Status;
    fn try_from(g: DelRequest) -> Result<Self, Self::Error> {
        let nochk = NoChecker::default();
        Self::new(g, &nochk)
    }
}
