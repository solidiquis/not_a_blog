use actix_web::web::ServiceConfig;

use super::super::controller::base;

/// Application routes
pub fn app_cfg(cfg: &mut ServiceConfig) {
    cfg
        .service(base::hello);
}
