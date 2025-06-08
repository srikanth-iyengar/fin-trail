use leptos::attr::Data;

use crate::{data_fetcher::DataFetcher, query_cache::QueryKey};
use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct MutationObserver {
    observers: HashMap<i32, DataFetcher>,
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

    pub(crate) fn get_data_fetcher(&self, key: i32) -> Option<DataFetcher> {
        self.observers.get(&key).map(|v| *v)
    }

    pub(crate) fn new() -> MutationObserver {
        MutationObserver {
            observers: HashMap::new(),
        }
    }
}

impl Default for MutationObserver {
    fn default() -> Self {
        Self::new()
    }
}
