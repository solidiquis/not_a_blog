use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool};
use std::time::Duration;

pub type DbConnPool = Pool<ConnectionManager<PgConnection>>;

/// Initializes the connection pool to Postgres. All configurations can be found in the .env
/// file. Pool size is calculated by taking the product of the number of CPU cores and DB_CONN_POOL_SIZE_PER_CORE.
/// Meant to be initialized first, and then cloned into each Actix worker.
pub fn init_conn_pool() -> DbConnPool {
    let database_uri = dotenv::var("DATABASE_URI").unwrap();
    let conn_manager = ConnectionManager::new(database_uri);

    let no_cpu_cores = num_cpus::get() as u32;
    let db_pool_size = u32::from_str_radix(&dotenv::var("DB_CONN_POOL_SIZE_PER_CORE").unwrap(), 10).unwrap() * no_cpu_cores;
    let conn_timeout = u64::from_str_radix(&dotenv::var("DB_CONNECTION_TIMEOUT_SECS").unwrap(), 10)
        .and_then(|timeout| Ok(Duration::from_secs(timeout)))
        .unwrap();

    log::info!("Postgres connection pool size: {:?}", &db_pool_size);
    log::info!("Postgres connection timeout: {:?}", &conn_timeout);

    r2d2::Pool::builder()
        .max_size(db_pool_size)
        .connection_timeout(conn_timeout)
        .build(conn_manager)
        .unwrap()
}
