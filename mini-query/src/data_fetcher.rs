use crate::query_cache::QueryValue;

pub trait DataFetcher {
    fn fetch(&self) -> Box<QueryValue>;
}
