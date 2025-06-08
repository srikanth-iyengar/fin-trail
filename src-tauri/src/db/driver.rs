use std::future::Future;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum DriverError {
    ConnectionError,
    NoRecordFound,
    UpdateError,
    UnknownError,
}

#[derive(Clone)]
pub struct Column<'a> {
    pub field_name: &'a str,
    pub is_primary_key: bool,
    pub is_not_null: bool,
    pub data_type: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Value {
    StringVal(String),
    Number(i64),
    Boolean(bool),
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Condition {
    pub field: String,
    pub operator: Operator,
}

pub trait Driver {
    fn create_table(
        &mut self,
        table_name: String,
        cols: Vec<Column>,
    ) -> impl Future<Output = Option<DriverError>>;
}

pub fn build_where_clause(conditions: Vec<Condition>) -> (String, Vec<Value>) {
    let mut clauses = Vec::new();

    let mut param_val = 1;
    let mut values: Vec<Value> = Vec::new();
    for cond in conditions {
        let clause = match cond.operator {
            Operator::Eq(val) => {
                values.push(val);
                param_val += 1;
                format!("{} = ${}", cond.field, param_val)
            }
            Operator::Neq(val) => {
                values.push(val);
                param_val += 1;
                format!("{} != ${}", cond.field, param_val)
            }
            Operator::Gt(val) => {
                values.push(val);
                param_val += 1;
                format!("{} > ${}", cond.field, param_val)
            }
            Operator::Gte(val) => {
                values.push(val);
                param_val += 1;
                format!("{} >= ${}", cond.field, param_val)
            }
            Operator::Lt(val) => {
                values.push(val);
                param_val += 1;
                format!("{} < ${}", cond.field, param_val)
            }
            Operator::Lte(val) => {
                values.push(val);
                param_val += 1;
                format!("{} <= ${}", cond.field, param_val)
            }
            Operator::Like(val) => {
                values.push(val);
                param_val += 1;
                format!("{} LIKE ${}", cond.field, param_val)
            }
            Operator::In(val) => {
                values.push(val);
                param_val += 1;
                format!("{} IN (${})", cond.field, param_val)
            }
            Operator::Between(start, end) => {
                values.push(start);
                values.push(end);
                param_val += 2;
                format!(
                    "{} BETWEEN ${} AND ${}",
                    cond.field,
                    param_val - 2,
                    param_val - 1
                )
            }
            Operator::IsNull => {
                format!("{} IS NULL", cond.field)
            }
            Operator::IsNotNull => {
                format!("{} IS NOT NULL", cond.field)
            }
        };
        clauses.push(clause);
    }

    if clauses.is_empty() {
        ("".to_string(), values)
    } else {
        (format!("WHERE {}", clauses.join(" AND ")), values)
    }
}

pub fn build_cols_query(cols: Vec<Column>) -> String {
    let mut clauses: Vec<String> = Vec::new();
    let mut primary_keys: Vec<String> = Vec::new();
    for col in cols.iter() {
        let mut clause = format!("{} {}", col.field_name, col.data_type);

        if col.is_not_null {
            clause.push_str(" NOT NULL");
        }

        clauses.push(clause);

        if col.is_primary_key {
            primary_keys.push(col.field_name.to_string());
        }
    }

    if !primary_keys.is_empty() {
        let pk_clause = format!("PRIMARY KEY ({})", primary_keys.join(", "));
        clauses.push(pk_clause);
    }

    clauses.join(",\n")
}

pub fn generate_create_table_query(table_name: String, cols: Vec<Column>) -> String {
    format!(
        "CREATE TABLE IF NOT EXISTS {} ({});",
        table_name,
        build_cols_query(cols)
    )
}
