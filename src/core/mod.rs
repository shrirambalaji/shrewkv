#![allow(dead_code)]

use std::collections::BTreeMap;

pub struct ShrewDB<K, V>
where
    K: Ord,
{
    btree_map: BTreeMap<K, V>,
}

impl<K, V> ShrewDB<K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        ShrewDB {
            btree_map: BTreeMap::new(),
        }
    }

    pub fn get(&self, key: K) -> Option<&V> {
        return self.btree_map.get(&key);
    }

    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        return self.btree_map.insert(key, value);
    }

    pub fn contains(&mut self, key: K) -> bool {
        return self.btree_map.contains_key(&key);
    }
}

// write tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_get() {
        let mut data_store = ShrewDB::new();
        data_store.put("key1", "value1");
        data_store.put("key2", "value2");
        assert_eq!(data_store.get(&"key1"), Some(&"value1"));
        assert_eq!(data_store.get(&"key2"), Some(&"value2"));
    }

    #[test]
    fn test_contains() {
        let mut data_store = ShrewDB::new();
        data_store.put("key1", "value1");
        assert_eq!(data_store.contains("key1"), true);
        assert_eq!(data_store.contains("key2"), false);
    }

    #[test]
    fn test_put_override() {
        let mut data_store = ShrewDB::new();
        data_store.put("key1", "value1");
        assert_eq!(data_store.put("key1", "value2"), Some("value1"));
        assert_eq!(data_store.get(&"key1"), Some(&"value2"));
    }
}
