use actix_http::{body::MessageBody, Request};
use actix_web::{
    dev::{Service, ServiceResponse},
    test,
};

pub async fn assert_request<B: MessageBody + std::fmt::Debug>(
    srv: &impl Service<Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
    path: &str,
    expected: &str,
) {
    let req = test::TestRequest::get().uri(path).to_request();
    let res = test::call_service(srv, req).await;
    assert!(res.status().is_success());
    let body = res.into_body().try_into_bytes().unwrap();
    let result = std::str::from_utf8(&body).unwrap();
    assert_eq!(result, expected);
}
