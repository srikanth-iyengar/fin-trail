use std::collections::HashMap;

use leptos::prelude::provide_context;

use crate::{
    data_fetcher::{AsyncDataFetcher, DataFetcher},
    mutation_observer::MutationObserver,
    query_cache::{QueryCache, QueryKey, QueryValue},
};

#[derive(Clone)]
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
        fetcher: DataFetcher,
    ) -> Option<&QueryValue> {
        let key = &query_key.key();
        self.mutation_observer.register_mutation(query_key, fetcher);

        if self.query_cache.contains(*key) {
            return self.query_cache.get(*key);
        }
        self.fetch_first_value(*key);
        self.query_cache.get(*key)
    }

    pub async fn register_data_fetcher_async(
        &mut self,
        query_key: Box<dyn QueryKey>,
        fetcher: AsyncDataFetcher,
    ) -> Option<&QueryValue> {
        let key = &query_key.key();
        self.mutation_observer
            .register_mutation_async(query_key, fetcher);

        if self.query_cache.contains(*key) {
            return self.query_cache.get(*key);
        }

        self.fetch_first_value_async(*key).await;
        self.query_cache.get(*key)
    }

    fn fetch_first_value(&mut self, key: i32) {
        if self.query_cache.cache.contains_key(&key) {
            return;
        }

        let data_fetcher = self.mutation_observer.get_data_fetcher(key);
        self.query_cache.put(key, Box::new(data_fetcher.unwrap()()));
    }

    async fn fetch_first_value_async(&mut self, key: i32) {
        if self.query_cache.cache.contains_key(&key) {
            return;
        }

        let data_fetcher = self.mutation_observer.get_data_fetcher_async(key);
        let value = data_fetcher.unwrap()().await;
        self.query_cache.put(key, Box::new(value));
    }

    pub fn invalidate_cache(&mut self, query_key: Box<dyn QueryKey>) {
        let key = query_key.key();

        let data_fetcher = self.mutation_observer.get_data_fetcher(key);

        self.query_cache.put(key, Box::new(data_fetcher.unwrap()()));
    }
}

impl Default for QueryClient {
    fn default() -> Self {
        Self::new()
    }
}
