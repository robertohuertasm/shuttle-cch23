mod common;

use actix_web::{test, App};
use cch23_robertohuertasm::app::configure_app;
use common::assert_request;

#[actix_web::test]
async fn day_01_works() {
    let app = App::new().configure(configure_app);
    let service = test::init_service(app).await;

    assert_request(&service, "/1/4/8", "1728").await;
    assert_request(&service, "/1/10", "1000").await;
    assert_request(&service, "/1/4/5/8/10", "27").await;
}
