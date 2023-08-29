use std::fmt;

use crate::rpc;

pub struct Uuid {
    raw: u128,
}

impl Uuid {
    pub fn new(hi: u64, lo: u64) -> Self {
        let h: u128 = hi.into();
        let l: u128 = lo.into();
        let raw: u128 = (h << 64) | l;
        Self { raw }
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:032x}", self.raw)
    }
}

impl From<&rpc::Uuid> for Uuid {
    fn from(g: &rpc::Uuid) -> Self {
        Self::new(g.hi, g.lo)
    }
}
