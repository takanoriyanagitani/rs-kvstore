use tonic::Status;

use crate::bucket::bkt::Bucket;
use crate::bucket::checker::{Checker, NoChecker};
use crate::uuid::Uuid;

use crate::rpc::DropRequest;

pub struct DropReq {
    request_id: Uuid,
    bucket: Bucket,
}

impl DropReq {
    pub fn new<C>(g: DropRequest, checker: &C) -> Result<Self, Status>
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
        Ok(Self { request_id, bucket })
    }

    pub fn as_request(&self) -> Uuid {
        self.request_id
    }

    pub fn as_bucket(&self) -> &Bucket {
        &self.bucket
    }
}

impl TryFrom<DropRequest> for DropReq {
    type Error = Status;
    fn try_from(g: DropRequest) -> Result<Self, Self::Error> {
        let nochk = NoChecker::default();
        Self::new(g, &nochk)
    }
}
