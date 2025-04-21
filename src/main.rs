use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::http::header;
use serde_json::Value;
use std::fs;

// Load the weather data from the JSON file at startup
fn load_weather_data() -> Value {
    let data = fs::read_to_string("weather.json").expect("Unable to read weather.json");
    serde_json::from_str(&data).expect("Invalid JSON format in weather.json")
}

// Root route: Redirect to /docs
async fn root() -> impl Responder {
    HttpResponse::MovedPermanently()
        .append_header((header::LOCATION, "/docs"))
        .finish()
}

// Countries route: Return a list of all available countries
async fn countries(weather_data: web::Data<Value>) -> impl Responder {
    let countries: Vec<String> = weather_data.as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect();
    HttpResponse::Ok().json(countries)
}

// Monthly average weather route
async fn monthly_weather(
    weather_data: web::Data<Value>,
    path: web::Path<(String, String, String)>,
) -> impl Responder {
    let (country, city, month) = path.into_inner();
    if let Some(city_data) = weather_data.get(&country).and_then(|c| c.get(&city)) {
        if let Some(month_data) = city_data.get(&month) {
            return HttpResponse::Ok().json(month_data);
        }
    }
    HttpResponse::NotFound().body("Weather data not found for the specified parameters")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let weather_data = load_weather_data();
    let weather_data = web::Data::new(weather_data);

    HttpServer::new(move || {
        App::new()
            .app_data(weather_data.clone())
            .route("/", web::get().to(root))
            .route("/countries", web::get().to(countries))
            .route("/countries/{country}/{city}/{month}", web::get().to(monthly_weather))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_root_route() {
        let app = test::init_service(App::new().route("/", web::get().to(root))).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::MOVED_PERMANENTLY);
        assert_eq!(
            resp.headers().get("location").unwrap(),
            "/docs"
        );
    }

    #[actix_web::test]
    async fn test_countries_route() {
        let weather_data = serde_json::json!({
            "Country1": {},
            "Country2": {}
        });
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(weather_data))
                .route("/countries", web::get().to(countries)),
        )
        .await;

        let req = test::TestRequest::get().uri("/countries").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        let body: Vec<String> = test::read_body_json(resp).await;
        assert_eq!(body, vec!["Country1", "Country2"]);
    }

    #[actix_web::test]
    async fn test_countries_not_found() {
        let weather_data = serde_json::json!({});
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(weather_data))
                .route("/countries", web::get().to(countries)),
        )
        .await;

        let req = test::TestRequest::get().uri("/countries").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        let body: Vec<String> = test::read_body_json(resp).await;
        assert_eq!(body, vec![]);
    }
    #[actix_web::test]
    async fn test_monthly_weather_route() {
        let weather_data = serde_json::json!({
            "Country1": {
                "City1": {
                    "January": { "temperature": 25.3, "humidity": 60, "precipitation": 12.5 }
                }
            }
        });
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(weather_data))
                .route("/countries/{country}/{city}/{month}", web::get().to(monthly_weather)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/countries/Country1/City1/January")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(
            body,
            serde_json::json!({
                "temperature": 25.3,
                "humidity": 60,
                "precipitation": 12.5
            })
        );
    }

    #[actix_web::test]
    async fn test_monthly_weather_route_not_found() {
        let weather_data = serde_json::json!({
            "Country1": {
                "City1": {
                    "January": { "temperature": 25.3, "humidity": 60, "precipitation": 12.5 }
                }
            }
        });
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(weather_data))
                .route("/countries/{country}/{city}/{month}", web::get().to(monthly_weather)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/countries/Country1/City1/February")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);
    }
}