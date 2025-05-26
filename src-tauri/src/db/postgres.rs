use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use super::driver::{generate_create_table_query, Driver, DriverError};

pub struct PostgresDriver {
    connection_string: String,
    pool: Pool<Postgres>,
}

impl PostgresDriver {
    pub async fn connect(conn_string: String) -> Result<PostgresDriver, DriverError> {
        let conn = PgPoolOptions::new()
            .max_connections(5)
            .connect(&conn_string)
            .await;
        println!("conn_string: {} {:?}", conn_string, conn);

        match conn {
            Ok(pool) => Ok(PostgresDriver {
                connection_string: conn_string,
                pool,
            }),
            Err(_) => Err(DriverError::ConnectionError {}),
        }
    }
}

impl Driver for PostgresDriver {
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
