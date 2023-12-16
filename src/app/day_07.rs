use actix_web::{web::Json, HttpRequest};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const NO_COOKIE: &str = "No Cookie";

pub async fn decode(req: HttpRequest) -> String {
    let cookie = req.cookie("recipe");
    if let Some(cookie) = cookie {
        return decode_impl(cookie.value()).await;
    }
    NO_COOKIE.to_string()
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct CookieData {
    recipe: HashMap<String, i64>,
    pantry: HashMap<String, i64>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct CookieResponse {
    pub cookies: i64,
    pub pantry: HashMap<String, i64>,
}

pub async fn bake(req: HttpRequest) -> Json<CookieResponse> {
    let cookie_value = decode(req).await;
    let result = bake_impl(&cookie_value).await;
    Json(result)
}

async fn bake_impl(cookie: &str) -> CookieResponse {
    if let Ok(cookie_data) = serde_json::from_str::<CookieData>(cookie) {
        // max number of cookies that we can bake
        let cookies = cookie_data
            .recipe
            .iter()
            .filter(|(_, &amount)| amount > 0)
            .enumerate()
            .fold(0, |acc, (i, (ingredient, amount))| {
                let pantry_amount = *cookie_data.pantry.get(ingredient).unwrap_or(&0);
                let cookie_quantity = pantry_amount.checked_div(*amount).unwrap_or(0);
                if i == 0 {
                    return cookie_quantity;
                }
                i64::min(acc, cookie_quantity)
            });

        // updated pantry
        let pantry =
            cookie_data
                .pantry
                .iter()
                .fold(HashMap::new(), |mut acc, (ingredient, amount)| {
                    let mut new_amount = *amount;
                    if let Some(recipe_amount) = cookie_data.recipe.get(ingredient) {
                        new_amount = amount - (recipe_amount * cookies);
                    }
                    acc.insert(ingredient.to_string(), new_amount);
                    acc
                });

        CookieResponse { cookies, pantry }
    } else {
        CookieResponse::default()
    }
}

async fn decode_impl(cookie: &str) -> String {
    // base64 decode cookie
    STANDARD
        .decode(cookie)
        .ok()
        .and_then(|x| String::from_utf8(x).ok())
        .unwrap_or_else(|| NO_COOKIE.to_string())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[actix_web::test]
    async fn bake_impl_works_with_0_values() {
        // {"recipe":{"cocoa bean":1,"chicken":0},"pantry":{"cocoa bean":5,"corn":5,"cucumber":0}}
        let cookie_value = "eyJyZWNpcGUiOnsiY29jb2EgYmVhbiI6MSwiY2hpY2tlbiI6MH0sInBhbnRyeSI6eyJjb2NvYSBiZWFuIjo1LCJjb3JuIjo1LCJjdWN1bWJlciI6MH19";

        let cookie_value = decode_impl(cookie_value).await;
        let result = bake_impl(&cookie_value).await;

        // {"cookies":5,"pantry":{"cocoa beam":0,"corn":5,"cucumber":0}
        let expected = CookieResponse {
            cookies: 5,
            pantry: {
                let mut map = HashMap::new();
                map.insert("cocoa beam".to_string(), 0);
                map.insert("corn".to_string(), 5);
                map.insert("cucumber".to_string(), 0);
                map
            },
        };
        assert_eq!(result, expected);
    }

    #[actix_web::test]
    async fn decode_impl_works() {
        let cookie_value = "eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ==";
        let result = decode_impl(cookie_value).await;
        let expected = r#"{"flour":100,"chocolate chips":20}"#;
        assert_eq!(result, expected);
    }

    #[actix_web::test]
    async fn decode_impl_works_2() {
        let cookie_value = "eyJyZWNpcGUiOnsiZmxvdXIiOjk1LCJzdWdhciI6NTAsImJ1dHRlciI6MzAsImJha2luZyBwb3dkZXIiOjEwLCJjaG9jb2xhdGUgY2hpcHMiOjUwfSwicGFudHJ5Ijp7ImZsb3VyIjozODUsInN1Z2FyIjo1MDcsImJ1dHRlciI6MjEyMiwiYmFraW5nIHBvd2RlciI6ODY1LCJjaG9jb2xhdGUgY2hpcHMiOjQ1N319";
        let result = decode_impl(cookie_value).await;
        let expected = r#"{"recipe":{"flour":95,"sugar":50,"butter":30,"baking powder":10,"chocolate chips":50},"pantry":{"flour":385,"sugar":507,"butter":2122,"baking powder":865,"chocolate chips":457}}"#;
        assert_eq!(result, expected);
    }

    #[actix_web::test]
    async fn decode_impl_returns_no_cookie_if_fails() {
        let cookie_value = "eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwf1111=";
        let result = decode_impl(cookie_value).await;
        let expected = NO_COOKIE;
        assert_eq!(result, expected);
    }

    #[actix_web::test]
    async fn bake_impl_works() {
        // {"recipe":{"flour":95,"sugar":50,"butter":30,"baking powder":10,"chocolate chips":50},"pantry":{"flour":385,"sugar":507,"butter":2122,"baking powder":865,"chocolate chips":457}}
        let cookie_value = "eyJyZWNpcGUiOnsiZmxvdXIiOjk1LCJzdWdhciI6NTAsImJ1dHRlciI6MzAsImJha2luZyBwb3dkZXIiOjEwLCJjaG9jb2xhdGUgY2hpcHMiOjUwfSwicGFudHJ5Ijp7ImZsb3VyIjozODUsInN1Z2FyIjo1MDcsImJ1dHRlciI6MjEyMiwiYmFraW5nIHBvd2RlciI6ODY1LCJjaG9jb2xhdGUgY2hpcHMiOjQ1N319";

        let cookie_value = decode_impl(cookie_value).await;
        let result = bake_impl(&cookie_value).await;

        // {"cookies":4,"pantry":{"flour":5,"sugar":307,"butter":2002,"baking powder":825,"chocolate chips":257}}
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
        assert_eq!(result, expected);
    }

    #[actix_web::test]
    async fn bake_impl_works_with_random_items() {
        // {"recipe":{"slime":9},"pantry":{"cobblestone":64,"stick": 4}}
        let cookie_value =
            "eyJyZWNpcGUiOnsic2xpbWUiOjl9LCJwYW50cnkiOnsiY29iYmxlc3RvbmUiOjY0LCJzdGljayI6IDR9fQ==";

        let cookie_value = decode_impl(cookie_value).await;
        let result = bake_impl(&cookie_value).await;

        // {"cookies":0,"pantry":{"cobblestone":64,"stick":4}}
        let expected = CookieResponse {
            cookies: 0,
            pantry: {
                let mut map = HashMap::new();
                map.insert("cobblestone".to_string(), 64);
                map.insert("stick".to_string(), 4);
                map
            },
        };
        assert_eq!(result, expected);
    }
}
