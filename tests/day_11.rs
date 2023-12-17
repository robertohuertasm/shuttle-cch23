mod common;

use actix_web::{
    test::{self, TestRequest},
    App,
};
use cch23_robertohuertasm::app::configure_app;
use common::get_response_body;

use crate::common::build_multipart_payload_and_header;

// curl -X POST localhost:8000/11/red_pixels --header 'Content-Type: multipart/form-data' --form 'image=@assets/decoration.png'

#[actix_web::test(skip)]
async fn integration_day_11_red_pixels() {
    let app = App::new().configure(configure_app);
    let service = test::init_service(app).await;

    let data = include_bytes!("../assets/decoration.png");
    // TODO: not working
    let data = String::from_utf8_lossy(data);

    let (payload, content_type_header) =
        build_multipart_payload_and_header("decoration.png", &data, "image", "image/png");

    let req = TestRequest::post()
        .uri("/11/red_pixels")
        .set_payload(payload)
        .insert_header(content_type_header)
        .to_request();

    let body = get_response_body(req, &service).await;
    let body: u32 = body.parse().unwrap();

    // Validation has a fault tolerance of 0.001
    let expected = 73034;
    assert!(expected == body, "body = {}", body);
}
