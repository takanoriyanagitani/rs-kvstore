use std::fmt;

use crate::rpc;

#[derive(Clone, Copy)]
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

    pub fn split(&self) -> (u64, u64) {
        let hi: u128 = self.raw >> 64;
        let lo: u128 = self.raw & 0xffff_ffff_ffff_ffff;
        (hi as u64, lo as u64)
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

impl From<Uuid> for rpc::Uuid {
    fn from(d: Uuid) -> Self {
        let (hi, lo) = d.split();
        Self { hi, lo }
    }
}
