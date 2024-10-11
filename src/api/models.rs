use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Clone)]
pub struct CreateCustomerAccount {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub id: i32,
}

#[derive(Deserialize, Clone)]
pub struct CreateAudioProduct {
    pub name: String,
    pub plugin_format: String,
    pub description: String,
    pub demo_href: String,
    pub image_src: String,
    pub image_alt: String,
    pub price: f32,
    pub category_id: i32,
}

#[derive(Deserialize, Clone)]
pub struct UpdateAudioProduct {
    pub name: String,
    pub description: String,
    pub plugin_format: String,
    pub demo_href: String,
    pub image_src: String,
    pub image_alt: String,
    pub price: f32,
    pub category_id: i32,
}

#[derive(Deserialize, Clone)]
pub struct CreateAudioCategory {
    pub name: String,
    pub products: Vec<i32>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UpdateAudioCategory {
    pub name: String,
    pub products: Vec<i32>,
}
