use actix_web::{App, HttpServer};
use crate::db::pg;
use std::io;
use super::lib::logger;

mod cfg;

/// Entry point for the entire application
pub async fn run() -> io::Result<()> {
    // Load data from .env
    dotenv::dotenv().ok();

    // Initialize global logger
    logger::init_global_logger();

    // Initialize Postgres connection pool
    let pg_conn_pool = pg::init_conn_pool();

    // Server basics
    let host = dotenv::var("HOST").unwrap();
    let port = dotenv::var("PORT").unwrap();
    let addr = format!["{}:{}", host, port];

    log::info!("Listening on tcp://{}", &addr);

    // Start the server
    HttpServer::new(move || {
        App::new()
            .configure(cfg::app_cfg)
            .app_data(pg_conn_pool.clone())
    })
        .bind(addr)?
        .run()
        .await
}

