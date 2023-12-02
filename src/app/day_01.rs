use actix_web::web;

pub async fn day_01(path: web::Path<String>) -> Option<String> {
    let value = path
        .into_inner()
        .split('/')
        .take(20)
        .filter_map(|s| s.parse::<i32>().ok())
        .reduce(|a, b| a ^ b)
        .map(|x| x.pow(3))
        .map(|x| x.to_string());
    value
}

#[cfg(test)]
mod tests {

    use super::*;

    #[actix_web::test]
    async fn day_01_works() {
        let path = web::Path::from("4/8".to_string());
        let result = day_01(path).await.unwrap();
        assert_eq!(result, "1728");

        let path = web::Path::from("10".to_string());
        let result = day_01(path).await.unwrap();
        assert_eq!(result, "1000");

        let path = web::Path::from("4/5/8/10".to_string());
        let result = day_01(path).await.unwrap();
        assert_eq!(result, "27");

        // 19 elements
        let path =
            web::Path::from("4/5/8/10/12/14/16/18/19/20/4/5/8/10/12/14/16/18/19".to_string());
        let result = day_01(path).await.unwrap();
        assert_eq!(result, "8000");

        // 20 elements
        let path =
            web::Path::from("4/5/8/10/12/14/16/18/19/20/4/5/8/10/12/14/16/18/19/1100".to_string());
        let result = day_01(path).await.unwrap();
        assert_eq!(result, "1375036928");

        // 20+ elements
        let path = web::Path::from(
            "4/5/8/10/12/14/16/18/19/20/4/5/8/10/12/14/16/18/19/1100/3230".to_string(),
        );
        let result = day_01(path).await.unwrap();
        assert_eq!(result, "1375036928");

        // wrong path won't take into account wrong elements
        let path = web::Path::from("4/5/8/10/1".to_string());
        let result = day_01(path).await.unwrap();
        assert_eq!(result, "8");

        let path = web::Path::from("4/5/8/10/abc/1".to_string());
        let result = day_01(path).await.unwrap();
        assert_eq!(result, "8");
    }
}
