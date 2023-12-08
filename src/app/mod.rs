mod day_00;
mod day_01;
mod day_04;
mod day_06;

pub use day_04::{ContestResult, Reinder};
pub use day_06::Day06Result;

use actix_web::web::{self, ServiceConfig};

async fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub fn configure_app(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(day_00::ok)))
        .service(web::resource("/-1/error").route(web::get().to(day_00::error_500)))
        .service(web::resource("/version").route(web::get().to(version)))
        .service(web::resource("/1/{all:.*}").route(web::get().to(day_01::day_01)))
        .service(web::resource("/4/strength").route(web::post().to(day_04::strength)))
        .service(web::resource("/4/contest").route(web::post().to(day_04::contest)))
        .service(web::resource("/6").route(web::post().to(day_06::day_06)));
}
