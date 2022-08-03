use std::collections::HashMap;

#[derive(Debug)]
pub struct KvStore {
    map: HashMap<String, String>
}

impl KvStore {
    pub fn new() -> Self {
        KvStore { map: HashMap::new() }
    }

    pub fn get(&self, key: String) -> Option<String> {
        match self.map.get(&key) {
            Some(val) => Some(val.to_owned()),
            None => None
        }
     }
    pub fn set(&mut self, key: String, val: String) {
        self.map.insert(key, val);
    }
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
