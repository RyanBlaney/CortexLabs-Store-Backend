use actix_web::{get, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    price: i32,
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

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(services::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
