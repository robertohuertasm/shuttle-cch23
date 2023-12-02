use actix_web::web::ServiceConfig;
use app::configure_app;
use shuttle_actix_web::ShuttleActixWeb;

mod app;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    Ok(configure_app.into())
}
