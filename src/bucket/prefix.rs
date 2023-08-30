use tonic::Status;

use crate::bucket::bkt::Bucket;

use crate::rpc::Key;

const JOIN_SEP_DEFAULT: u8 = b'/';

pub trait BucketAsPrefix {
    fn key_with_bucket(&self, bucket: &Bucket, key: &Key) -> Result<Key, Status>;
}

struct Concat {}

impl BucketAsPrefix for Concat {
    fn key_with_bucket(&self, bucket: &Bucket, key: &Key) -> Result<Key, Status> {
        let b: &[u8] = bucket.as_bytes();
        let k: &[u8] = &key.k;
        let bk: &[u8] = &[b, k].concat();
        let v: Vec<u8> = bk.into();
        Ok(Key { k: v })
    }
}

struct Join {
    j: u8,
}

impl BucketAsPrefix for Join {
    fn key_with_bucket(&self, bucket: &Bucket, key: &Key) -> Result<Key, Status> {
        let b: &[u8] = bucket.as_bytes();
        let k: &[u8] = &key.k;
        let bjk: &[u8] = &[b, &[self.j], k].concat();
        let v: Vec<u8> = bjk.into();
        Ok(Key { k: v })
    }
}

pub fn bucket_as_prefix_new_concat() -> impl BucketAsPrefix {
    Concat {}
}

pub fn bucket_as_prefix_new_join(sep: u8) -> impl BucketAsPrefix {
    Join { j: sep }
}

pub fn bucket_as_prefix_new_join_default() -> impl BucketAsPrefix {
    bucket_as_prefix_new_join(JOIN_SEP_DEFAULT)
}
