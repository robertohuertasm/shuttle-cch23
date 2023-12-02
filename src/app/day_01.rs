use actix_web::web;

pub async fn day_01(path: web::Path<String>) -> String {
    let value = path
        .into_inner()
        .split('/')
        .map(|s| s.parse::<i32>().unwrap())
        .take(20)
        .reduce(|a, b| a ^ b)
        .map(|x| x.pow(3))
        .map(|x| x.to_string())
        .unwrap_or_else(|| "0".to_string());
    value
}

#[cfg(test)]
mod tests {

    use super::*;

    #[actix_web::test]
    async fn day_01_works() {
        let path = web::Path::from("4/8".to_string());
        let result = day_01(path).await;
        assert_eq!(result, "1728");

        let path = web::Path::from("10".to_string());
        let result = day_01(path).await;
        assert_eq!(result, "1000");

        let path = web::Path::from("4/5/8/10".to_string());
        let result = day_01(path).await;
        assert_eq!(result, "27");
    }
}
