use actix_web::HttpResponse;

pub async fn ok() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn error_500() -> HttpResponse {
    HttpResponse::InternalServerError().finish()
}

#[cfg(test)]
mod tests {

    use actix_http::StatusCode;

    use super::*;

    #[actix_web::test]
    async fn ok_works() {
        let result = ok().await;
        assert_eq!(result.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn error_500_works() {
        let result = error_500().await;
        assert_eq!(result.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
