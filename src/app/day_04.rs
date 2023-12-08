use actix_http::body::EitherBody;
use actix_web::{
    error::JsonPayloadError,
    web::{self, Json},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct MiniReinder {
    name: String,
    strength: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Reinder {
    pub name: String,
    pub strength: u32,
    pub speed: f32,
    pub height: u32,
    pub antler_width: u32,
    pub snow_magic_power: u32,
    pub favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    pub candies_eaten_yesterday: u32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct ContestResult {
    pub fastest: String,
    pub tallest: String,
    pub magician: String,
    pub consumer: String,
}

impl ContestResult {
    fn new(fastest: &Reinder, tallest: &Reinder, magician: &Reinder, consumer: &Reinder) -> Self {
        ContestResult {
            fastest: format!(
                "Speeding past the finish line with a strength of {} is {}",
                fastest.strength, fastest.name
            ),
            tallest: format!(
                "{} is standing tall with his {} cm wide antlers",
                tallest.name, tallest.antler_width
            ),
            magician: format!(
                "{} could blast you away with a snow magic power of {}",
                magician.name, magician.snow_magic_power
            ),
            consumer: format!(
                "{} ate lots of candies, but also some {}",
                consumer.name, consumer.favorite_food
            ),
        }
    }
}

impl Responder for ContestResult {
    type Body = <Json<Self> as Responder>::Body;

    fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        Json(self).respond_to(req)
    }
}

pub async fn strength(reinders: Json<Vec<Reinder>>) -> String {
    println!("reinders {}", reinders.len());
    reinders.iter().map(|r| r.strength).sum::<u32>().to_string()
}

pub async fn contest(reinders: Json<Vec<Reinder>>) -> ContestResult {
    let first = reinders
        .first()
        .expect("One participant is always guaranteed");
    let mut fastest: &Reinder = first;
    let mut tallest: &Reinder = first;
    let mut magician: &Reinder = first;
    let mut consumer: &Reinder = first;

    for reinder in reinders.iter() {
        if reinder.speed > fastest.speed {
            fastest = reinder;
        }
        if reinder.height > tallest.height {
            tallest = reinder;
        }
        if reinder.snow_magic_power > magician.snow_magic_power {
            magician = reinder;
        }
        if reinder.candies_eaten_yesterday > consumer.candies_eaten_yesterday {
            consumer = reinder;
        }
    }

    ContestResult::new(fastest, tallest, magician, consumer)
}

#[cfg(test)]
mod tests {

    use super::*;

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
    async fn strength_works() {
        let reinders = vec![
            simple_reinder("Dasher".to_string(), 5),
            simple_reinder("Dancer".to_string(), 6),
            simple_reinder("Prancer".to_string(), 4),
            simple_reinder("Vixen".to_string(), 7),
        ];
        let payload = Json(reinders);
        let result = strength(payload).await;
        assert_eq!(result, "22");
    }

    #[actix_web::test]
    async fn contest_works() {
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
        let payload = Json(reinders);
        let result = contest(payload).await;
        assert_eq!(
            result,
            ContestResult {
                fastest: "Speeding past the finish line with a strength of 5 is Dasher".to_string(),
                tallest: "Dasher is standing tall with his 36 cm wide antlers".to_string(),
                magician: "Dasher could blast you away with a snow magic power of 9001".to_string(),
                consumer: "Dancer ate lots of candies, but also some grass".to_string()
            }
        );
    }
}
