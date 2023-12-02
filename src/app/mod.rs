mod day_00;
mod day_01;

use actix_web::web::{self, ServiceConfig};

async fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub fn configure_app(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(day_00::ok)))
        .service(web::resource("/-1/error").route(web::get().to(day_00::error_500)))
        .service(web::resource("/version").route(web::get().to(version)))
        .service(web::resource("/1/{all:.*}").route(web::get().to(day_01::day_01)));
}
