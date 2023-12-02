use actix_http::StatusCode;
use actix_web::{test, App};
use cch23_robertohuertasm::app::configure_app;

#[actix_web::test]
async fn day_00_works() {
    let app = App::new().configure(configure_app);
    let service = test::init_service(app).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let res = test::call_service(&service, req).await;
    assert_eq!(res.status(), StatusCode::OK);

    let req = test::TestRequest::get().uri("/-1/error").to_request();
    let res = test::call_service(&service, req).await;
    assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
