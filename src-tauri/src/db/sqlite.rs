use sqlx::{Connection, SqliteConnection, SqlitePool};

use super::driver::{generate_create_table_query, Driver, DriverError};

pub struct SqliteDriver {
    connection_string: String,
    pub pool: SqlitePool,
}

impl SqliteDriver {
    pub async fn connect(conn_string: String) -> Result<SqliteDriver, DriverError> {
        let conn =
            SqlitePool::connect("sqlite:///data/data/in.srikanthk.finmanager/fin-manager.db").await;

        match conn {
            Ok(pool) => Ok(SqliteDriver {
                pool,
                connection_string: conn_string,
            }),
            Err(err) => {
                eprintln!("{err}");
                Err(DriverError::ConnectionError {})
            }
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
        let result = sqlx::query(query.as_str()).execute(&self.pool).await;
        match result {
            Ok(_) => None,
            Err(_) => Some(DriverError::UnknownError),
        }
    }
}
