use crate::{
    data_fetcher::{AsyncDataFetcher, DataFetcher},
    query_cache::{QueryKey, QueryValue},
    query_client::QueryClient,
};
use leptos::{
    prelude::{ReadSignal, Set, signal, use_context},
    task::spawn_local,
};

pub enum QueryState {
    Loading,
    Invalid,
    Completed,
}

pub fn use_query(
    query_fn: DataFetcher,
    query_key: Box<dyn QueryKey>,
) -> (ReadSignal<Option<QueryValue>>, ReadSignal<QueryState>) {
    let mut query_client = use_context::<QueryClient>().unwrap();

    let (data, set_data) = signal::<Option<QueryValue>>(None);
    let (query_state, set_query_state) = signal::<QueryState>(QueryState::Loading);

    let first_value = query_client.register_data_fetcher(query_key, query_fn);

    set_data.set(first_value.cloned());

    (data, query_state)
}

pub fn use_query_async(
    query_fn: AsyncDataFetcher,
    query_key: Box<dyn QueryKey>,
) -> (ReadSignal<Option<QueryValue>>, ReadSignal<QueryState>) {
    let mut query_client = use_context::<QueryClient>().unwrap();
    let (data, set_data) = signal::<Option<QueryValue>>(None);
    let (query_state, set_query_state) = signal::<QueryState>(QueryState::Loading);

    spawn_local(async move {
        let first_value = query_client
            .register_data_fetcher_async(query_key, query_fn)
            .await;
        set_data.set(first_value.cloned());
    });

    (data, query_state)
}
