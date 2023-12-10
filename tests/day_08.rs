mod common;

use crate::common::assert_content_type;
use actix_web::{
    test::{self, TestRequest},
    App,
};
use cch23_robertohuertasm::app::configure_app;
use common::{assert_request_get, get_response_body, ContentType};

#[actix_web::test]
async fn integration_day_08_weight_works() {
    let app = App::new().configure(configure_app);
    let service = test::init_service(app).await;

    assert_request_get(&service, "/8/weight/25", "6").await;
    assert_request_get(&service, "/8/weight/40", "12").await;
    assert_request_get(&service, "/8/weight/1", "6.9").await;
}

#[actix_web::test]
async fn integration_day_08_weight_returns_plain_text_value() {
    let app = App::new().configure(configure_app);
    let service = test::init_service(app).await;
    let req = TestRequest::get().uri("/8/weight/25").to_request();
    assert_content_type(req, &service, ContentType::PlainText).await;
}

#[actix_web::test]
async fn integration_day_08_drop_works() {
    let app = App::new().configure(configure_app);
    let service = test::init_service(app).await;
    let req = TestRequest::get().uri("/8/drop/25").to_request();

    let body = get_response_body(req, &service).await;
    let body: f32 = body.parse().unwrap();

    // Validation has a fault tolerance of 0.001
    let expected = 84.10707461325713;
    let expected_limits = (expected - 0.001, expected + 0.001);
    assert!(
        body >= expected_limits.0 && body <= expected_limits.1,
        "body = {}",
        body
    );
}

#[actix_web::test]
async fn integration_day_08_drop_returns_plain_text_value() {
    let app = App::new().configure(configure_app);
    let service = test::init_service(app).await;
    let req = TestRequest::get().uri("/8/drop/25").to_request();
    assert_content_type(req, &service, ContentType::PlainText).await;
}
