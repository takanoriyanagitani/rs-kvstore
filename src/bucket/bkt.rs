use tonic::Status;

use crate::rpc;

use crate::bucket::checker::Checker;

pub struct Bucket {
    checked: String,
}

impl Bucket {
    pub fn new<C>(unchecked: String, checker: C) -> Result<Self, Status>
    where
        C: Checker,
    {
        let checked: String = checker.check(unchecked)?;
        Ok(Self { checked })
    }
}

impl From<Bucket> for rpc::Bucket {
    fn from(d: Bucket) -> Self {
        Self { b: d.checked }
    }
}
