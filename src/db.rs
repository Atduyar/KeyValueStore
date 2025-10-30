#![allow(dead_code, unused)]
use crate::kv_value::KVValue;
use std::collections::HashMap;

type Key = String;
type Value = KVValue;

pub struct DB(HashMap<Key, Value>);

impl DB {
    pub fn new() -> DB {
        DB(HashMap::new())
    }

    pub fn get(&self, key: &Key) -> Option<&Value> {
        self.0.get(key)
    }

    pub fn insert(&mut self, key: Key, value: Value) -> Option<Value> {
        self.0.insert(key, value)
    }

    pub fn remove(&mut self, key: &Key) -> Option<Value> {
        self.0.remove(key)
    }
}
