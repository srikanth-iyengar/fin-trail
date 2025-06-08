use std::{any::Any, collections::HashMap, sync::Arc};

pub trait QueryKey {
    fn key(&self) -> i32;
}

pub type QueryValue = Arc<dyn Any + Send + Sync>;

#[derive(Clone)]
pub struct QueryCache {
    pub(crate) cache: HashMap<i32, Box<QueryValue>>,
}

impl QueryCache {
    // Corrected function signature
    pub(crate) fn put_in_cache(&mut self, wrapper: Box<dyn QueryKey>, value: Box<QueryValue>) {
        let key = wrapper.key();
        self.put(key, value);
    }

    pub(crate) fn put(&mut self, key: i32, value: Box<QueryValue>) {
        self.cache.insert(key, value);
    }

    // This function already uses the correct, idiomatic pattern
    pub fn get_from_cache(&self, wrapper: Box<dyn QueryKey>) -> Option<&QueryValue> {
        let key = wrapper.key();
        self.get(key)
    }

    pub fn get(&self, key: i32) -> Option<&QueryValue> {
        self.cache.get(&key).map(|v| &**v)
    }

    pub fn contains(&self, key: i32) -> bool {
        self.cache.contains_key(&key)
    }
}
