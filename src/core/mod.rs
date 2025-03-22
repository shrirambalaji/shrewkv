#![allow(dead_code)]

use concread::bptree::BptreeMap;
use std::fmt::Debug;

pub struct ShrewKV<K, V>
where
    K: Ord + Debug + Clone + Sync + Send + 'static,
    V: Ord + Debug + Clone + Sync + Send + 'static,
{
    tree: BptreeMap<K, V>,
}

impl<K, V> ShrewKV<K, V>
where
    K: Ord + Debug + Clone + Sync + Send + 'static,
    V: Ord + Debug + Clone + Sync + Send + 'static,
{
    pub fn new() -> Self {
        ShrewKV {
            tree: BptreeMap::new(),
        }
    }

    pub fn get(&mut self, key: &K) -> V {
        let reader = self.tree.read();
        let value = reader.get(key);
        return value.unwrap().clone();
    }

    pub fn set(&self, key: K, value: V) -> Option<V> {
        let mut writer = self.tree.write();
        return writer.insert(key, value);
    }

    pub fn contains(&self, key: K) -> bool {
        let reader = self.tree.read();
        return reader.contains_key(&key);
    }
}

// write tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_get() {
        let mut data_store = ShrewKV::new();
        data_store.set("key1", "value1");
        data_store.set("key2", "value2");
        assert_eq!(data_store.get(&"key1"), "value1");
        assert_eq!(data_store.get(&"key2"), "value2");
    }

    #[test]
    fn test_contains() {
        let data_store = ShrewKV::new();
        data_store.set("key1", "value1");
        assert_eq!(data_store.contains("key1"), true);
        assert_eq!(data_store.contains("key2"), false);
    }

    #[test]
    fn test_put_override() {
        let mut data_store = ShrewKV::new();
        data_store.set("key1", "value1");
        assert_eq!(data_store.set("key1", "value2"), Some("value1"));
        assert_eq!(data_store.get(&"key1"), "value2");
    }
}
