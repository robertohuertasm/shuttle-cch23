mod common;

use actix_web::{
    test::{self, TestRequest},
    App,
};
use cch23_robertohuertasm::app::{configure_app, Day06Result};
use common::{assert_content_type, assert_request_post, ContentType};

#[actix_web::test]
async fn integration_day_06_works() {
    let app = App::new().configure(configure_app);
    let service = test::init_service(app).await;

    let input = "there is an elf on a shelf on an elf.
    there is also another shelf in Belfast."
        .to_string();

    let expected_result = Day06Result {
        elf: 5,
        elf_on_a_shelf: 1,
        shelf_with_no_elf_on_it: 1,
    };

    let expected = serde_json::to_string(&expected_result).unwrap();

    assert_request_post(&service, "/6", input, expected.as_str()).await;
}

#[actix_web::test]
async fn integration_day_06_works_2() {
    let app = App::new().configure(configure_app);
    let service = test::init_service(app).await;

    let input = "The mischievous elf peeked out from behind the toy workshop,
    and another elf joined in the festive dance.
    Look, there is also an elf on that shelf!"
        .to_string();

    let expected_result = Day06Result {
        elf: 4,
        elf_on_a_shelf: 0,
        shelf_with_no_elf_on_it: 1,
    };

    let expected = serde_json::to_string(&expected_result).unwrap();

    assert_request_post(&service, "/6", input, expected.as_str()).await;
}

#[actix_web::test]
async fn integration_day_06_returns_json_value() {
    let app = App::new().configure(configure_app);
    let service = test::init_service(app).await;

    let input = "there is an elf on a shelf on an elf.
    there is also another shelf in Belfast."
        .to_string();

    let req = TestRequest::post().uri("/6").set_json(input).to_request();
    assert_content_type(req, &service, ContentType::Json).await;
}
