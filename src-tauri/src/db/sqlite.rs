use sqlx::{Connection, SqliteConnection};

use super::driver::{generate_create_table_query, Driver, DriverError};

pub struct SqliteDriver {
    connection_string: String,
    conn: SqliteConnection,
}

impl SqliteDriver {
    pub async fn connect(conn_string: String) -> Result<SqliteDriver, DriverError> {
        let conn = SqliteConnection::connect("fin-manager.db").await;

        match conn {
            Ok(conn) => Ok(SqliteDriver {
                conn,
                connection_string: conn_string,
            }),
            Err(_) => Err(DriverError::ConnectionError {}),
        }
    }
}

impl Driver for SqliteDriver {
    async fn create_table(
        &mut self,
        table_name: String,
        cols: Vec<super::driver::Column<'_>>,
    ) -> Option<DriverError> {
        let query = generate_create_table_query(table_name, cols);
        let result = sqlx::query(query.as_str()).execute(&mut self.conn).await;
        match result {
            Ok(_) => None,
            Err(_) => Some(DriverError::UnknownError),
        }
    }
}
