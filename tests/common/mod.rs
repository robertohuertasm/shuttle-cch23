#![allow(dead_code)]

use actix_http::{body::MessageBody, Request};
use actix_web::{
    dev::{Service, ServiceResponse},
    test::{self, TestRequest},
};
use serde::Serialize;

pub async fn get_response_body<B: MessageBody + std::fmt::Debug>(
    req: Request,
    srv: &impl Service<Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
) -> String {
    let res = test::call_service(srv, req).await;
    assert!(res.status().is_success());
    let body = res.into_body().try_into_bytes().unwrap();
    std::str::from_utf8(&body).unwrap().to_string()
}

pub async fn assert_request<B: MessageBody + std::fmt::Debug>(
    req: Request,
    srv: &impl Service<Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
    expected: &str,
) {
    let result = get_response_body(req, srv).await;
    assert_eq!(result, expected);
}

pub async fn assert_request_get<B: MessageBody + std::fmt::Debug>(
    srv: &impl Service<Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
    path: &str,
    expected: &str,
) {
    let req = TestRequest::get().uri(path).to_request();
    assert_request(req, srv, expected).await;
}

pub async fn assert_request_post<B: MessageBody + std::fmt::Debug, T: Serialize>(
    srv: &impl Service<Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
    path: &str,
    payload: T,
    expected: &str,
) {
    let req = TestRequest::post().uri(path).set_json(payload).to_request();
    assert_request(req, srv, expected).await;
}
