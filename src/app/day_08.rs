use actix_web::web::Path;
use serde::{Deserialize, Serialize};

const POKEAPI_URL_POKEMON: &str = "https://pokeapi.co/api/v2/pokemon";
const G: f32 = 9.825;

#[derive(Debug, Deserialize, Serialize)]
struct Pokemon {
    id: u32,
    name: String,
    weight: u32,
}

pub async fn weight(pokemon_id: Path<String>) -> String {
    let weight = weight_impl(pokemon_id.into_inner()).await;
    float_to_string(weight)
}

pub async fn drop(pokemon_id: Path<String>) -> String {
    let weight = weight_impl(pokemon_id.into_inner()).await;
    let momentum = calculate_momentum(weight);
    float_to_string(momentum)
}

async fn get_pokemon(pokemon_id: String) -> Option<Pokemon> {
    reqwest::get(format!("{}/{}", POKEAPI_URL_POKEMON, pokemon_id))
        .await
        .map(|r| r.json::<Pokemon>())
        .ok()?
        .await
        .ok()
}

async fn weight_impl(pokemon_id: String) -> f32 {
    if let Some(pokemon) = get_pokemon(pokemon_id).await {
        pokemon.weight as f32 / 10.0
    } else {
        0.0
    }
}

fn float_to_string(value: f32) -> String {
    format!("{}", value)
}

fn calculate_momentum(weight: f32) -> f32 {
    let chimney_height = 10.0;
    // (2ad).sqrt()
    let velocity = (2.0 * G * chimney_height).sqrt();
    // momentum = mass * velocity
    weight * velocity
}

#[cfg(test)]
mod tests {

    use super::*;

    #[actix_web::test]
    async fn weight_impl_works() {
        let w = weight_impl("25".to_string()).await;
        assert_eq!(w, 6.0);
    }

    #[actix_web::test]
    async fn float_to_string_should_remove_dot_zero() {
        let w = float_to_string(6.0);
        assert_eq!(w, "6");
    }

    #[actix_web::test]
    async fn float_to_string_should_keep_dot_if_not_zero() {
        let w = float_to_string(6.1);
        assert_eq!(w, "6.1");
    }

    #[actix_web::test]
    async fn calculate_momentum_works() {
        let m = calculate_momentum(6.0);
        // Validation has a fault tolerance of 0.001
        let expected = 84.10707461325713;
        let expected_limits = (expected - 0.001, expected + 0.001);
        assert!(
            m >= expected_limits.0 && m <= expected_limits.1,
            "m = {}",
            m
        );
    }
}
