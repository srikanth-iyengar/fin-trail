use leptos_struct_table::*;
use serde::{Deserialize, Serialize};

#[derive(TableRow, Clone, Serialize, Deserialize)]
#[table(impl_vec_data_provider)]
pub struct Transaction {
    pub tx_id: String,
    pub ts: i64,
    pub amount: f64,
    pub direction: bool,
    pub is_synced: bool,
    pub tags: String,
    pub acc_id: String,
}
