mod app;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    app::server::run().await
}
