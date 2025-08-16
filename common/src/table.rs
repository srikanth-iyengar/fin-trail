use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Value {
    StringVal(String),
    Number(i64),
    Boolean(bool),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Operator {
    Eq(Value),
    Neq(Value),
    Gt(Value),
    Gte(Value),
    Lt(Value),
    Lte(Value),
    Like(Value),
    In(Value),
    Between(Value, Value),
    IsNull,
    IsNotNull,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Condition {
    pub field: String,
    pub operator: Operator,
}

#[derive(Clone)]
pub struct Column<'a> {
    pub field_name: &'a str,
    pub is_primary_key: bool,
    pub is_not_null: bool,
    pub data_type: &'a str,
}

pub const TRANSACTION_TB: &str = "fin_transaction";
pub const ACCOUNT_TB: &str = "fin_account";
pub const REC_TX_TB: &str = "fin_recurrenttransaction";

pub const DOUBLE: &str = "double precision";

#[derive(Serialize, Deserialize, sqlx::FromRow, Clone, Debug)]
pub struct Transaction {
    pub tx_id: String,
    pub ts: i64,
    pub amount: f64,
    pub direction: bool,
    pub is_synced: bool,
    pub tags: String,
    pub acc_id: String,
}

pub struct Account {
    acc_id: String,
}

pub const TX_TABLE: [Column; 7] = [
    Column {
        field_name: "tx_id",
        data_type: "varchar(100)",
        is_not_null: false,
        is_primary_key: true,
    },
    Column {
        field_name: "ts",
        data_type: "bigint",
        is_not_null: false,
        is_primary_key: false,
    },
    Column {
        field_name: "amount",
        data_type: "double precision",
        is_not_null: false,
        is_primary_key: false,
    },
    Column {
        field_name: "direction",
        data_type: "boolean",
        is_primary_key: false,
        is_not_null: false,
    },
    Column {
        field_name: "is_synced",
        data_type: "boolean",
        is_primary_key: false,
        is_not_null: false,
    },
    Column {
        field_name: "tags",
        data_type: "varchar(255)",
        is_primary_key: false,
        is_not_null: false,
    },
    Column {
        field_name: "acc_id",
        data_type: "varchar(100)",
        is_primary_key: false,
        is_not_null: false,
    },
];

pub const ACC_TABLE: [Column; 4] = [
    Column {
        field_name: "acc_id",
        data_type: "varchar(100)",
        is_primary_key: true,
        is_not_null: false,
    },
    Column {
        field_name: "acc_name",
        data_type: "varchar(100)",
        is_primary_key: false,
        is_not_null: false,
    },
    Column {
        field_name: "balance",
        data_type: DOUBLE,
        is_primary_key: false,
        is_not_null: false,
    },
    Column {
        field_name: "color",
        data_type: "varchar(40)",
        is_primary_key: false,
        is_not_null: false,
    },
];

pub const REC_TX_TABLE: [Column; 4] = [
    Column {
        field_name: "cron_expr",
        data_type: "varchar(40)",
        is_primary_key: false,
        is_not_null: false,
    },
    Column {
        field_name: "amount",
        data_type: DOUBLE,
        is_not_null: false,
        is_primary_key: false,
    },
    Column {
        field_name: "direction",
        data_type: "boolean",
        is_not_null: false,
        is_primary_key: false,
    },
    Column {
        field_name: "acc_id",
        data_type: "boolean",
        is_not_null: false,
        is_primary_key: false,
    },
];
