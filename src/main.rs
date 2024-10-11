use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpServer};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod api;
use api::services;

struct AppState {
    audio_products: Mutex<Vec<AudioProduct>>,
    audio_categories: Mutex<Vec<AudioCategory>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct CustomerAccount {
    first_name: String,
    last_name: String,
    email: String,
    id: i32,
    premium_plan: bool,
}

#[derive(Serialize, Deserialize, Clone)]
struct AudioCategory {
    id: i32,
    name: String,
    products: Vec<AudioProduct>,
}

#[derive(Serialize, Deserialize, Clone)]
struct AudioProduct {
    id: i32,
    name: String,
    description: String,
    plugin_format: String,
    demo_href: String,
    image_src: String,
    image_alt: String,
    price: f32,
    category_id: i32,
}

#[get("/")]
async fn index() -> String {
    "This is a health check".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        audio_products: Mutex::new(vec![]),
        audio_categories: Mutex::new(vec![]),
    });

    dotenv().ok();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
                    .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT])
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(app_data.clone())
            .service(index)
            .configure(services::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
