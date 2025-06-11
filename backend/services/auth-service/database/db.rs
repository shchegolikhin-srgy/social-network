use tokio_postgres::{NoTls};
use bb8::Pool;
use bb8_tokio_postgres::PostgresConnectionManager;
use crate::core::config::db_config;


pub async fn initialize_db_pool()-> Result<(), Box<dyn std::error::Error>>{
    // ЗАМЕНИТЬ НА КОНФИГ 
    let config = "host=localhost user=postgres password=secret dbname=testdb port=5432";
    let mgr = PostgresConnectionManager::new_from_stringlike(config, NoTls)?;
    let pool = Pool::builder().build_unchecked(mgr);
}
pub async fn get_db_client(pool: &Pool<PostgresConnectionManager<NoTls>>) -> Result<Client, Infallible> {
    let client = pool.get().await.expect("Failed to get client from pool");
    Ok(client)
}
pub async fn get_users()->Result<String, Error>{
    client = get_db_client(&pool);
}