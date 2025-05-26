use std::{fmt::Display, future::Future};

pub enum DriverError {
    ConnectionError,
    NoRecordFound,
    UpdateError,
    UnknownError,
}

pub struct Column<'a> {
    pub field_name: &'a str,
    pub is_primary_key: bool,
    pub is_not_null: bool,
    pub data_type: &'a str,
}

#[derive(Debug)]
pub enum Operator {
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    Like,
    In,
    Between,
    IsNull,
    IsNotNull,
}

#[derive(Debug)]
pub struct Condition<'a> {
    pub field: &'a str,
    pub operator: Operator,
}

pub trait Driver {
    fn create_table(
        &mut self,
        table_name: String,
        cols: Vec<Column>,
    ) -> impl Future<Output = Option<DriverError>>;
}

pub fn build_where_clause(conditions: Vec<Condition>) -> String {
    let mut clauses = Vec::new();

    for cond in conditions {
        let clause = match cond.operator {
            Operator::Eq => format!("{} = ?", cond.field),
            Operator::Neq => format!("{} != ?", cond.field),
            Operator::Gt => format!("{} > ?", cond.field),
            Operator::Gte => format!("{} >= ?", cond.field),
            Operator::Lt => format!("{} < ?", cond.field),
            Operator::Lte => format!("{} <= ?", cond.field),
            Operator::Like => format!("{} LIKE ?", cond.field),
            Operator::In => format!("{} IN (?)", cond.field),
            Operator::Between => format!("{} BETWEEN ? AND ?", cond.field,),
            Operator::IsNull => format!("{} IS NULL", cond.field),
            Operator::IsNotNull => format!("{} IS NOT NULL", cond.field),
        };
        clauses.push(clause);
    }

    if clauses.is_empty() {
        "".to_string()
    } else {
        format!("WHERE {}", clauses.join(" AND "))
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

pub fn generate_select_table_query(table_name: String, where_cond: Vec<Condition>) -> String {
    format!(
        "SELECT * FROM {} {}",
        table_name,
        build_where_clause(where_cond)
    )
}

pub fn generate_delete_query<T: Display>(table_name: String, where_cond: Vec<Condition>) -> String {
    format!(
        "DELETE FROM {} {}",
        table_name,
        build_where_clause(where_cond)
    )
}
