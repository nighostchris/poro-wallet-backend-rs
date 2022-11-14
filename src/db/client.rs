use log::info;
use sqlx::{postgres::PgPool, Pool, Postgres};

pub async fn init_db_client(db_conn_url: String) -> Pool<Postgres> {
    let db_client = PgPool::connect(db_conn_url.as_str()).await;
    match db_client {
        Ok(client) => {
            info!("Successfully initiated database connection.");
            client
        }
        Err(e) => panic!("Cannot initiate database connection. {}", e),
    }
}
