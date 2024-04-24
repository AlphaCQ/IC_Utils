use std::collections::HashMap;
use std::hash::Hash;
use std::collections::{LinkedList};

// Custom implementation of a LinkedHashMap
struct LinkedHashMap<K, V> {
    map: HashMap<K, V>,
    list: LinkedList<K>,
}


impl<K, V> LinkedHashMap<K, V>
    where
        K: Eq + Hash + Clone,
{
    fn new() -> Self {
        LinkedHashMap {
            map: HashMap::new(),
            list: LinkedList::new(),
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(old_value) = self.map.insert(key.clone(), value) {
            return Some(old_value);
        }
        self.list.push_back(key);
        None
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(value) = self.map.remove(key) {
            self.list = self.list.iter().cloned().filter(|k| k != key).collect();
            return Some(value);
        }
        None
    }

    fn iter(&self) -> std::collections::hash_map::Iter<K, V> {
        self.map.iter()
    }
}
// 定义一个结构体来存储对象
pub struct MemoryStorage {
    data: LinkedHashMap<String, Vec<u8>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        MemoryStorage {
            data: LinkedHashMap::new(),
        }
    }

    pub fn write(&mut self, key: String, data: Vec<u8>) {
        self.data.insert(key, data);
    }

    pub fn read(&self, key: String) -> Option<Vec<u8>> {
        self.data.get(&key).cloned()
    }

    pub fn delete(&mut self, key: String) -> Option<Vec<u8>> {
        self.data.remove(&key)
    }

    pub fn list(&self) -> Vec<LinkedList<u8>> {
        let mut list = Vec::new();
        for (_, value) in self.data.iter() {
            list.push(value.iter().cloned().collect::<LinkedList<u8>>());
        }
        list
    }
}