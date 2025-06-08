use crate::query_cache::QueryValue;

pub type DataFetcher = fn() -> QueryValue;

// pub trait DataFetcher {
//     fn fetch(&self) -> Box<QueryValue>;
// }
