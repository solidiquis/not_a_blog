use actix_web::{get, HttpRequest, HttpResponse, Responder};
use actix_web::web::Data;
use crate::db::pg::DbConnPool;

#[get("/")]
async fn hello(req: HttpRequest, db: Data<DbConnPool>) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
