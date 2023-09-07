use std::collections::BTreeMap;

use crate::bucket::bkt::Bucket;

use crate::internal::core::KeyValue;

use crate::rpc::{Key, Val};

pub struct BTree<K> {
    internal: BTreeMap<Bucket, K>,
}

impl<K> KeyValue for BTree<K>
where
    K: KeyValue + Default,
{
    fn get(&self, b: &Bucket, k: &Key) -> Option<Val> {
        match self.internal.get(b) {
            None => None,
            Some(kvstore) => kvstore.get(b, k),
        }
    }

    fn exists(&self, b: &Bucket) -> bool {
        self.internal.contains_key(b)
    }

    fn exists_key(&self, b: &Bucket, k: &Key) -> bool {
        match self.internal.get(b) {
            None => false,
            Some(kvstore) => kvstore.exists_key(b, k),
        }
    }

    fn set(&mut self, b: Bucket, k: Key, v: Val) -> Option<Val> {
        let mut kvstore: K = self.internal.remove(&b).unwrap_or_default();
        let ov: Option<_> = kvstore.set(b.clone(), k, v);
        self.internal.insert(b, kvstore);
        ov
    }

    fn del(&mut self, b: &Bucket, k: &Key) -> Option<Val> {
        match self.internal.get_mut(b) {
            None => None,
            Some(kvstore) => kvstore.del(b, k),
        }
    }

    fn truncate(&mut self, b: &Bucket) {
        match self.internal.get_mut(b) {
            None => {}
            Some(kvstore) => {
                kvstore.truncate(b);
            }
        }
    }
}

pub fn kv_btree_new<K>() -> impl KeyValue
where
    K: KeyValue + Default,
{
    BTree::<K> {
        internal: BTreeMap::default(),
    }
}

pub fn kv_btree_btree_new() -> impl KeyValue {
    kv_btree_new::<BTreeMap<Vec<u8>, Val>>()
}
