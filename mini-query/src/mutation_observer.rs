use crate::{
    data_fetcher::{AsyncDataFetcher, DataFetcher},
    query_cache::QueryKey,
};
use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct MutationObserver {
    observers: HashMap<i32, DataFetcher>,
    async_observers: HashMap<i32, AsyncDataFetcher>,
}

impl MutationObserver {
    pub(crate) fn register_mutation(
        &mut self,
        query_key: Box<dyn QueryKey>,
        data_fetcher: DataFetcher,
    ) -> Option<DataFetcher> {
        let key = query_key.key();
        if self.observers.contains_key(&key) {
            self.observers.insert(key, data_fetcher);
        }

        self.get_data_fetcher(key)
    }

    pub(crate) fn register_mutation_async(
        &mut self,
        query_key: Box<dyn QueryKey>,
        data_fetcher: AsyncDataFetcher,
    ) -> Option<AsyncDataFetcher> {
        let key = query_key.key();
        if self.async_observers.contains_key(&key) {
            self.async_observers.insert(key, data_fetcher);
        }

        self.get_data_fetcher_async(key)
    }

    pub(crate) fn get_data_fetcher(&self, key: i32) -> Option<DataFetcher> {
        self.observers.get(&key).copied()
    }

    pub(crate) fn get_data_fetcher_async(&self, key: i32) -> Option<AsyncDataFetcher> {
        self.async_observers.get(&key).copied()
    }

    pub(crate) fn new() -> MutationObserver {
        MutationObserver {
            observers: HashMap::new(),
            async_observers: HashMap::new(),
        }
    }
}

impl Default for MutationObserver {
    fn default() -> Self {
        Self::new()
    }
}
