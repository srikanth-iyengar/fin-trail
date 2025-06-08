use crate::{
    data_fetcher::DataFetcher,
    query_cache::{QueryKey, QueryValue},
    query_client::QueryClient,
};
use leptos::prelude::{ReadSignal, Set, signal, use_context};

pub fn use_query(
    query_fn: DataFetcher,
    query_key: Box<dyn QueryKey>,
) -> (ReadSignal<Option<QueryValue>>) {
    let mut query_client = use_context::<QueryClient>().unwrap();

    let (data, set_data) = signal::<Option<QueryValue>>(None);

    let first_value = query_client.register_data_fetcher(query_key, query_fn);

    set_data.set(first_value.cloned());

    data
}
