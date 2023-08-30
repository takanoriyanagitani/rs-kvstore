use tonic::Status;

use crate::rpc;

use crate::bucket::checker::Checker;

#[derive(Default)]
pub struct Bucket {
    checked: String,
}

impl Bucket {
    pub fn new<C>(unchecked: String, checker: &C) -> Result<Self, Status>
    where
        C: Checker,
    {
        let checked: String = checker.check(unchecked)?;
        Ok(Self { checked })
    }

    pub fn as_str(&self) -> &str {
        self.checked.as_str()
    }
    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}

impl From<Bucket> for rpc::Bucket {
    fn from(d: Bucket) -> Self {
        Self { b: d.checked }
    }
}
