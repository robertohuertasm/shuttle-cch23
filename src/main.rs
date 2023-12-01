use actix_web::web::{self, ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;

async fn cch23_1(path: web::Path<String>) -> String {
    let value = path
        .split('/')
        .map(|s| s.parse::<i32>().unwrap())
        .take(20)
        .reduce(|a, b| a ^ b)
        .map(|x| x.pow(3))
        .map(|x| x.to_string())
        .unwrap_or_else(|| "0".to_string());
    value
}

async fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(web::resource("/1/{all:.*}").route(web::get().to(cch23_1)))
            .service(web::resource("/version").route(web::get().to(version)));
    };

    Ok(config.into())
}
