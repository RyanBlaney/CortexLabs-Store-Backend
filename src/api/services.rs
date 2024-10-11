use super::models::{
    CreateAudioCategory, CreateAudioProduct, UpdateAudioCategory, UpdateAudioProduct,
};
use crate::{AppState, AudioCategory, AudioProduct};
use actix_web::{
    delete, get, patch, post,
    web::{self},
    HttpResponse, Responder,
};

// AudioProducts
#[get("/plugin_store/products")]
async fn get_audio_products(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.audio_products.lock().unwrap().to_vec())
}

#[get("/plugin_store/products/{id}")]
async fn get_audio_products_by_id(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    println!("Fetching product with ID: {}", id);

    let products = data.audio_products.lock().unwrap();
    for product in &*products {
        if product.id == id {
            return HttpResponse::Ok().json(product);
        }
    }

    println!("Product with ID {} not found", id);
    HttpResponse::NotFound().finish()
}

#[post("/plugin_store/products")]
async fn create_audio_product(
    data: web::Data<AppState>,
    param_obj: web::Json<CreateAudioProduct>,
) -> impl Responder {
    let new_product;
    {
        let mut products = data.audio_products.lock().unwrap();
        let max_id: i32 = products.iter().map(|p| p.id).max().unwrap_or(0);

        new_product = AudioProduct {
            name: param_obj.name.clone(),
            id: max_id + 1,
            price: param_obj.price.clone(),
            description: param_obj.description.clone(),
            plugin_format: param_obj.plugin_format.clone(),
            demo_href: param_obj.demo_href.clone(),
            image_src: param_obj.image_src.clone(),
            image_alt: param_obj.image_alt.clone(),
            category_id: param_obj.category_id.clone(),
        };

        products.push(new_product.clone());
    } // Lock is released here

    // Step 2: Update the associated category asynchronously
    if new_product.category_id > 0 {
        if let Err(e) = update_category_with_product(new_product.category_id, new_product.id).await
        {
            eprintln!(
                "Failed to update category {}: {}",
                new_product.category_id, e
            );
            return HttpResponse::InternalServerError()
                .body(format!("Failed to update category: {}", e));
        }
    }

    // Step 3: Return the updated products list
    let products = data.audio_products.lock().unwrap().to_vec();
    HttpResponse::Ok().json(products)
}

async fn update_category_with_product(
    category_id: i32,
    product_id: i32,
) -> Result<(), reqwest::Error> {
    let mut category = fetch_category_by_id(category_id).await?;

    // Add the new product ID to the category's product list if it's not already there
    if !category.products.iter().any(|p| p.id == product_id) {
        category.products.push(AudioProduct {
            id: product_id,
            name: String::new(),
            price: 0.0,
            description: String::new(),
            plugin_format: String::new(),
            demo_href: "".to_string(),
            image_src: "".to_string(),
            image_alt: "".to_string(),
            category_id: category_id.clone(),
        });
    }

    // Create the updated category payload
    let updated_category = UpdateAudioCategory {
        name: category.name.clone(),
        products: category.products.iter().map(|p| p.id).collect(),
    };

    let payload = serde_json::to_value(&updated_category)
        .expect("Failed to serialize category update payload");

    // Send the patch request to update the category
    update_category(category_id, &payload).await
}

#[patch("/plugin_store/products/{id}")]
async fn update_audio_product(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    param_obj: web::Json<UpdateAudioProduct>,
) -> impl Responder {
    let id = path.into_inner();
    let mut products = data.audio_products.lock().unwrap();
    for i in 0..products.len() {
        if products[i].id == id {
            products[i].name = param_obj.name.clone();
            products[i].description = param_obj.description.clone();
            products[i].plugin_format = param_obj.plugin_format.clone();
            products[i].price = param_obj.price.clone();
            products[i].demo_href = param_obj.demo_href.clone();
            products[i].image_src = param_obj.image_src.clone();
            products[i].image_alt = param_obj.image_alt.clone();
            products[i].category_id = param_obj.category_id.clone();

            println!("Category to update: {}", products[i].category_id);

            //Update Category
            // Fetch Category by ID
            let category = fetch_category_by_id(products[i].category_id).await.unwrap();

            // Clone category
            let mut prod_ids: Vec<i32> = [].to_vec();
            for prod in category.products {
                prod_ids.push(prod.id);
            }

            let new_category = UpdateAudioCategory {
                name: String::from(category.name),
                products: prod_ids,
            };

            let payload =
                serde_json::to_value(&new_category).expect("Failed to serialize category");

            // Create patch request
            if let Err(e) = update_category(category.id, &payload).await {
                return HttpResponse::InternalServerError()
                    .body(format!("Failed to update category: {}", e));
            }

            break;
        }
    }

    HttpResponse::Ok().json(products.to_vec())
}

async fn update_category(
    category_id: i32,
    json_to_send: &serde_json::Value,
) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!(
        "http://localhost:8080/plugin_store/categories/{}",
        category_id
    );
    let response = client
        .patch(&url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&json_to_send)
        .send()
        .await?;
    response.error_for_status()?;
    Ok(())
}

#[delete("/plugin_store/products/{id}")]
async fn delete_audio_product(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let mut products = data.audio_products.lock().unwrap();

    let id = path.into_inner();
    *products = products
        .to_vec()
        .into_iter()
        .filter(|x| x.id != id)
        .collect();

    HttpResponse::Ok().json(products.to_vec())
}

// AudioCategory

async fn fetch_category_by_id(id: i32) -> Result<AudioCategory, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client
        .get(format!(
            "http://localhost:8080/plugin_store/categories/{}",
            id
        ))
        .send()
        .await?;

    if response.status().is_success() {
        let category: AudioCategory = response.json().await?;
        Ok(category)
    } else {
        println!(
            "Failed to fetch category with ID {}: {}",
            id,
            response.status()
        );
        Err(reqwest::Error::from(
            response.error_for_status().unwrap_err(),
        ))
    }
}

async fn fetch_products_from_ids(
    product_ids: Vec<i32>,
) -> Result<Vec<AudioProduct>, reqwest::Error> {
    let mut products = Vec::new();

    for id in product_ids {
        let response = reqwest::get(format!(
            "http://localhost:8080/plugin_store/products/{}",
            id
        ))
        .await?
        .text()
        .await?;

        let product: AudioProduct = serde_json::from_str(&response).unwrap();
        products.push(product)

        // if response.get {
        // let product: AudioProduct = response.json().await?;

        // } else {
        // println!(
        // "Failed to fetch product with ID {}: {}",
        // id,
        // response.status()
        // );
        // }
    }

    Ok(products)
}

#[get("/plugin_store/categories")]
async fn get_audio_categories(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.audio_categories.lock().unwrap().to_vec())
}

#[get("/plugin_store/categories/{id}")]
async fn get_audio_category_by_id(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    println!("Fetching category with ID: {}", id);

    let categories = data.audio_categories.lock().unwrap();
    for cat in &*categories {
        if cat.id == id {
            return HttpResponse::Ok().json(cat);
        }
    }

    println!("Product with ID {} not found", id);
    HttpResponse::NotFound().finish()
}

#[post("/plugin_store/categories")]
async fn create_audio_category(
    data: web::Data<AppState>,
    param_obj: web::Json<CreateAudioCategory>,
) -> impl Responder {
    let mut categories = data.audio_categories.lock().unwrap();
    let max_id: i32 = categories.iter().map(|cat| cat.id).max().unwrap_or(0);

    let mut list_products: Vec<AudioProduct> = fetch_products_from_ids(param_obj.products.clone())
        .await
        .unwrap_or_else(|err| {
            eprintln!("Failed to fetch products: {}", err);
            Vec::new()
        });

    for prod in &mut list_products {
        prod.category_id = max_id + 1;
    }

    categories.push(AudioCategory {
        name: param_obj.name.clone(),
        id: max_id + 1,
        products: list_products.clone(),
    });

    // Update the category_id for each product in the audio_products list
    let mut products = data.audio_products.lock().unwrap();
    for prod in &mut *products {
        if list_products.iter().any(|p| p.id == prod.id) {
            prod.category_id = max_id + 1;
        }
    }

    HttpResponse::Ok().json(categories.to_vec())
}

#[patch("/plugin_store/categories/{id}")]
async fn update_audio_category(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    param_obj: web::Json<UpdateAudioCategory>,
) -> impl Responder {
    let id = path.into_inner();
    let mut categories = data.audio_categories.lock().unwrap();
    let mut products = fetch_products_from_ids(param_obj.products.clone())
        .await
        .unwrap();

    println!("We successfully cloned the product data! :)");

    for prod in products.iter_mut() {
        prod.category_id = id;
    }

    for i in 0..categories.len() {
        if categories[i].id == id {
            categories[i].name = param_obj.name.clone();
            categories[i].products = products;
            break;
        }
    }

    HttpResponse::Ok().json(categories.to_vec())
}

#[delete("/plugin_store/categories/{id}")]
async fn delete_audio_category(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let mut categories = data.audio_categories.lock().unwrap();

    let id = path.into_inner();
    *categories = categories
        .to_vec()
        .into_iter()
        .filter(|x| x.id != id)
        .collect();

    HttpResponse::Ok().json(categories.to_vec())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_audio_products)
        .service(get_audio_products_by_id)
        .service(create_audio_product)
        .service(update_audio_product)
        .service(delete_audio_product)
        .service(get_audio_categories)
        .service(get_audio_category_by_id)
        .service(create_audio_category)
        .service(update_audio_category)
        .service(delete_audio_category);
}
