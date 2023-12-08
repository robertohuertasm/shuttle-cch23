use actix_web::{web::Json, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Day06Result {
    pub elf: u32,
    #[serde(rename = "elf on a shelf")]
    pub elf_on_a_shelf: u32,
    #[serde(rename = "shelf with no elf on it")]
    pub shelf_with_no_elf_on_it: u32,
}

impl Responder for Day06Result {
    type Body = <Json<Self> as Responder>::Body;

    fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        Json(self).respond_to(req)
    }
}

pub async fn day_06(input: String) -> Day06Result {
    let elf = input.matches("elf").count() as u32;
    let elf_on_a_shelf = input.matches("elf on a shelf").count() as u32;
    let shelf_with_no_elf_on_it = input.matches("shelf").count() as u32 - elf_on_a_shelf;

    Day06Result {
        elf,
        elf_on_a_shelf,
        shelf_with_no_elf_on_it,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[actix_web::test]
    async fn it_works() {
        let input = "there is an elf on a shelf on an elf.
        there is also another shelf in Belfast."
            .to_string();

        let result = day_06(input).await;
        assert_eq!(
            result,
            Day06Result {
                elf: 5,
                elf_on_a_shelf: 1,
                shelf_with_no_elf_on_it: 1
            }
        );
    }

    #[actix_web::test]
    async fn it_works_2() {
        let input = "The mischievous elf peeked out from behind the toy workshop,
        and another elf joined in the festive dance.
        Look, there is also an elf on that shelf!"
            .to_string();

        let result = day_06(input).await;
        assert_eq!(
            result,
            Day06Result {
                elf: 4,
                elf_on_a_shelf: 0,
                shelf_with_no_elf_on_it: 1
            }
        );
    }
}
