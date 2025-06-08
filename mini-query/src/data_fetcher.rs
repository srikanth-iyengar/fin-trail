use std::pin::Pin;

use crate::query_cache::QueryValue;

pub type DataFetcher = fn() -> QueryValue;
pub type AsyncDataFetcher = fn() -> Pin<Box<dyn Future<Output = QueryValue>>>;
