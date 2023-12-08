mod common;

use std::collections::HashMap;

use actix_web::{
    test::{self, TestRequest},
    App,
};
use cch23_robertohuertasm::app::{configure_app, CookieResponse};
use common::{assert_request, assert_request_get, get_response_body};

#[actix_web::test]
async fn integration_day_07_decode_with_cookie_works() {
    let app = App::new().configure(configure_app);
    let service = test::init_service(app).await;

    let req = TestRequest::get()
        .uri("/7/decode")
        .insert_header((
            "Cookie",
            "recipe=eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ==",
        ))
        .to_request();

    let expected = r#"{"flour":100,"chocolate chips":20}"#;

    assert_request(req, &service, expected).await;
}

#[actix_web::test]
async fn integration_day_07_decode_without_cookie_works() {
    let app = App::new().configure(configure_app);
    let service = test::init_service(app).await;
    let expected = "No Cookie";
    assert_request_get(&service, "/7/decode", expected).await;
}

#[actix_web::test]
async fn integration_day_07_bake_with_cookie_works() {
    let app = App::new().configure(configure_app);
    let service = test::init_service(app).await;

    let req = TestRequest::get()
        .uri("/7/bake")
        .insert_header((
            "Cookie",
            "recipe=eyJyZWNpcGUiOnsiZmxvdXIiOjk1LCJzdWdhciI6NTAsImJ1dHRlciI6MzAsImJha2luZyBwb3dkZXIiOjEwLCJjaG9jb2xhdGUgY2hpcHMiOjUwfSwicGFudHJ5Ijp7ImZsb3VyIjozODUsInN1Z2FyIjo1MDcsImJ1dHRlciI6MjEyMiwiYmFraW5nIHBvd2RlciI6ODY1LCJjaG9jb2xhdGUgY2hpcHMiOjQ1N319",
        ))
        .to_request();

    // {"recipe":{"flour":95,"sugar":50,"butter":30,"baking powder":10,"chocolate chips":50},"pantry":{"flour":385,"sugar":507,"butter":2122,"baking powder":865,"chocolate chips":457}}

    // we're not comparing string as hashmap order is not guaranteed
    let expected = CookieResponse {
        cookies: 4,
        pantry: {
            let mut map = HashMap::new();
            map.insert("flour".to_string(), 5);
            map.insert("sugar".to_string(), 307);
            map.insert("butter".to_string(), 2002);
            map.insert("baking powder".to_string(), 825);
            map.insert("chocolate chips".to_string(), 257);
            map
        },
    };

    // {"cookies":4,"pantry":{"flour":5,"sugar":307,"butter":2002,"baking powder":825,"chocolate chips":257}}

    let response_body = get_response_body(req, &service).await;
    let response_cookie = serde_json::from_str::<CookieResponse>(&response_body).unwrap();

    assert_eq!(response_cookie, expected);
}

#[actix_web::test]
async fn integration_day_07_bake_with_cookie_and_random_items_works() {
    let app = App::new().configure(configure_app);
    let service = test::init_service(app).await;

    let req = TestRequest::get()
        .uri("/7/bake")
        .insert_header((
            "Cookie",
            "recipe=eyJyZWNpcGUiOnsic2xpbWUiOjl9LCJwYW50cnkiOnsiY29iYmxlc3RvbmUiOjY0LCJzdGljayI6IDR9fQ==",
        ))
        .to_request();

    // {"recipe":{"slime":9},"pantry":{"cobblestone":64,"stick": 4}}

    // we're not comparing string as hashmap order is not guaranteed
    let expected = CookieResponse {
        cookies: 0,
        pantry: {
            let mut map = HashMap::new();
            map.insert("cobblestone".to_string(), 64);
            map.insert("stick".to_string(), 4);
            map
        },
    };

    // {"cookies":0,"pantry":{"cobblestone":64,"stick":4}}

    let response_body = get_response_body(req, &service).await;
    let response_cookie = serde_json::from_str::<CookieResponse>(&response_body).unwrap();

    assert_eq!(response_cookie, expected);
}
