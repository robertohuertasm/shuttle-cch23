mod day_00;
mod day_01;
mod day_04;
mod day_06;
mod day_07;
mod day_08;
mod day_11;

pub use day_04::{ContestResult, Reinder};
pub use day_06::Day06Result;
pub use day_07::CookieResponse;

use actix_web::web::{self, ServiceConfig};

async fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub fn configure_app(cfg: &mut ServiceConfig) {
    let bytes_limit = actix_web::web::FormConfig::default().limit(1024 * 1024 * 10);

    cfg.service(web::resource("/").route(web::get().to(day_00::ok)))
        .service(web::resource("/-1/error").route(web::get().to(day_00::error_500)))
        .service(web::resource("/version").route(web::get().to(version)))
        .service(web::resource("/1/{all:.*}").route(web::get().to(day_01::day_01)))
        .service(web::resource("/4/strength").route(web::post().to(day_04::strength)))
        .service(web::resource("/4/contest").route(web::post().to(day_04::contest)))
        .service(web::resource("/6").route(web::post().to(day_06::day_06)))
        .service(web::resource("/7/decode").route(web::get().to(day_07::decode)))
        .service(web::resource("/7/bake").route(web::get().to(day_07::bake)))
        .service(web::resource("/8/weight/{weight}").route(web::get().to(day_08::weight)))
        .service(web::resource("/8/drop/{pokedex_number}").route(web::get().to(day_08::drop)))
        .service(
            web::resource("/11/assets/decoration.png").route(web::get().to(day_11::decoration)),
        )
        .service(
            web::resource("/11/red_pixels")
                .app_data(bytes_limit)
                .route(web::post().to(day_11::red_pixels)),
        );
}
