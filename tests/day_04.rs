mod common;

use actix_web::{
    test::{self, TestRequest},
    App,
};
use cch23_robertohuertasm::app::{configure_app, ContestResult, Reinder};
use common::{
    assert_content_type, assert_request_post, assert_request_post_direct_payload, ContentType,
};
use std::vec;

fn simple_reinder(name: String, strength: u32) -> Reinder {
    Reinder {
        name,
        strength,
        speed: 0.0,
        height: 0,
        antler_width: 0,
        snow_magic_power: 0,
        favorite_food: "".to_string(),
        candies_eaten_yesterday: 0,
    }
}

#[actix_web::test]
async fn integration_day_04_strength_works() {
    let app = App::new().configure(configure_app);
    let service = test::init_service(app).await;

    let reinders = vec![
        simple_reinder("Dasher".to_string(), 5),
        simple_reinder("Dancer".to_string(), 6),
        simple_reinder("Prancer".to_string(), 4),
        simple_reinder("Vixen".to_string(), 7),
    ];

    assert_request_post(&service, "/4/strength", reinders, "22").await;
}

#[actix_web::test]
async fn integration_day_04_strength_works_2() {
    let app = App::new().configure(configure_app);
    let service = test::init_service(app).await;

    let reinders = r#"[
     {
        "name": "Dasher",
        "strength": 5
      },
      {
        "name": "Dancer",
        "strength": 6
      },
      {
        "name": "Prancer",
        "strength": 4
      },
      {
        "name": "Vixen",
        "strength": 7
      }
   ]"#
    .to_string();

    assert_request_post_direct_payload(&service, "/4/strength", reinders, "22").await;
}

#[actix_web::test]
async fn integration_day_04_strength_returns_plain_text_value() {
    let app = App::new().configure(configure_app);
    let service = test::init_service(app).await;

    let input = vec![
        simple_reinder("Dasher".to_string(), 5),
        simple_reinder("Dancer".to_string(), 6),
        simple_reinder("Prancer".to_string(), 4),
        simple_reinder("Vixen".to_string(), 7),
    ];

    let req = TestRequest::post()
        .uri("/4/strength")
        .set_json(input)
        .to_request();
    assert_content_type(req, &service, ContentType::PlainText).await;
}

#[actix_web::test]
async fn integration_day_04_contest_works() {
    let app = App::new().configure(configure_app);
    let service = test::init_service(app).await;

    let reinders = vec![
        Reinder {
            name: "Dasher".to_string(),
            strength: 5,
            speed: 50.4,
            height: 80,
            antler_width: 36,
            snow_magic_power: 9001,
            favorite_food: "hay".to_string(),
            candies_eaten_yesterday: 2,
        },
        Reinder {
            name: "Dancer".to_string(),
            strength: 6,
            speed: 48.2,
            height: 65,
            antler_width: 37,
            snow_magic_power: 4004,
            favorite_food: "grass".to_string(),
            candies_eaten_yesterday: 5,
        },
    ];

    let expected_result = ContestResult {
        fastest: "Speeding past the finish line with a strength of 5 is Dasher".to_string(),
        tallest: "Dasher is standing tall with his 36 cm wide antlers".to_string(),
        magician: "Dasher could blast you away with a snow magic power of 9001".to_string(),
        consumer: "Dancer ate lots of candies, but also some grass".to_string(),
    };

    let expected = serde_json::to_string(&expected_result).unwrap();

    assert_request_post(&service, "/4/contest", reinders, expected.as_str()).await;
}

#[actix_web::test]
async fn integration_day_04_contest_returns_plain_text_value() {
    let app = App::new().configure(configure_app);
    let service = test::init_service(app).await;

    let input = vec![
        simple_reinder("Dasher".to_string(), 5),
        simple_reinder("Dancer".to_string(), 6),
    ];

    let req = TestRequest::post()
        .uri("/4/contest")
        .set_json(input)
        .to_request();

    assert_content_type(req, &service, ContentType::Json).await;
}
