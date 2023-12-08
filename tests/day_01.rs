mod common;

use actix_web::{test, App};
use cch23_robertohuertasm::app::configure_app;
use common::assert_request_get;

#[actix_web::test]
async fn integration_day_01_works() {
    let app = App::new().configure(configure_app);
    let service = test::init_service(app).await;

    assert_request_get(&service, "/1/4/8", "1728").await;
    assert_request_get(&service, "/1/10", "1000").await;
    assert_request_get(&service, "/1/4/5/8/10", "27").await;
    assert_request_get(
        &service,
        "/1/4/5/8/10/12/14/16/18/19/20/4/5/8/10/12/14/16/18/19/1100",
        "1375036928",
    )
    .await;
    assert_request_get(
        &service,
        "/1/4/5/8/10/12/14/16/18/19/20/4/5/8/10/12/14/16/18/19/1100/3230",
        "1375036928",
    )
    .await;
    assert_request_get(&service, "/1/4/5/8/10/1", "8").await;
    assert_request_get(&service, "/1/4/5/8/10/abc/1", "8").await;
}
