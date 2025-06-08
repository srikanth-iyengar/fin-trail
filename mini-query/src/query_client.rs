use std::collections::HashMap;

use leptos::prelude::{StorageAccess, provide_context};

use crate::{
    mutation_observer::{MutationObserver, SafeFetcher},
    query_cache::{QueryCache, QueryKey, QueryValue},
};

pub struct QueryClient {
    pub(crate) query_cache: QueryCache,
    pub(crate) mutation_observer: MutationObserver,
}

impl QueryClient {
    pub fn new() -> QueryClient {
        QueryClient {
            query_cache: QueryCache {
                cache: HashMap::new(),
            },
            mutation_observer: MutationObserver::new(),
        }
    }

    pub fn provide_query_client() {
        provide_context(QueryClient::new());
    }

    pub fn register_data_fetcher(
        &mut self,
        query_key: Box<dyn QueryKey>,
        fetcher: Box<SafeFetcher>,
    ) -> Option<&QueryValue> {
        let key = &query_key.key();
        self.mutation_observer.register_mutation(query_key, fetcher);

        if self.query_cache.contains(*key) {
            return self.query_cache.get(*key);
        }
        self.fetch_first_value(*key);
        self.query_cache.get(*key)
    }

    pub fn fetch_first_value(&mut self, key: i32) {
        if self.query_cache.cache.contains_key(&key) {
            return;
        }

        let data_fetcher = self.mutation_observer.get_data_fetcher(key);
        let value = data_fetcher.unwrap().fetch();
        self.query_cache.put(key, value);
    }
}

impl Default for QueryClient {
    fn default() -> Self {
        Self::new()
    }
}
