use actix_http::{body::MessageBody, Request};
use actix_web::{
    dev::{Service, ServiceResponse},
    test,
};
use serde::Serialize;

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

pub async fn assert_request_post<B: MessageBody + std::fmt::Debug, T: Serialize>(
    srv: &impl Service<Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
    path: &str,
    payload: T,
    expected: &str,
) {
    let req = test::TestRequest::post()
        .uri(path)
        .set_json(payload)
        .to_request();
    let res = test::call_service(srv, req).await;
    assert!(res.status().is_success());
    let body = res.into_body().try_into_bytes().unwrap();
    let result = std::str::from_utf8(&body).unwrap();
    assert_eq!(result, expected);
}
