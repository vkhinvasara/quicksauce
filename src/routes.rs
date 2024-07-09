use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemInput, GetItemInput};
use actix_web::{post, web, HttpResponse, Responder, get};
use serde::Deserialize;
use crate::utils::shorten_url;
use crate::models::Sauce;
use std::collections::HashMap;
use dotenv::dotenv;
use std::env;
pub struct AppState {
   pub dynamodb_client: DynamoDbClient,
}

#[derive(Deserialize)]
struct UrlPayload {
	url: String,
}

#[post("/create_url")]
async fn create_url(url: web::Json<UrlPayload>, data: web::Data<AppState>) -> impl Responder {
    let url = url.into_inner().url;
    let client = &data.dynamodb_client;
    let short_url = shorten_url(&url);
    let item = Sauce::new(url.to_string(), short_url.to_string());
    let mut attribute_values = HashMap::new();
    attribute_values.insert("id".to_string(), AttributeValue { s: Some(item.id.to_string()), ..Default::default() });
    attribute_values.insert("url".to_string(), AttributeValue { s: Some(item.url.to_string()), ..Default::default() });
    attribute_values.insert("short_url".to_string(), AttributeValue { s: Some(item.short_url.to_string()), ..Default::default() });
    let input = PutItemInput {
        table_name: "sauces".to_string(),
        item: attribute_values,
        condition_expression: Some("attribute_not_exists(id)".to_string()),
        ..Default::default()
    };
    match client.put_item(input).await {
        Ok(_) => HttpResponse::Ok().body(short_url.clone()),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error creating short URL: {:?}", e)),
    }
}


#[get("/{id}")]
async fn redirect(id: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    dotenv().ok(); // Load environment variables
    let base_url = env::var("BASE_URL").expect("BASE_URL must be set in .env file");
    let full_url = format!("{}{}", base_url, id.into_inner()); // Construct the full URL

    let client = &data.dynamodb_client;
    let mut key = HashMap::new();
    key.insert("full_url".to_string(), AttributeValue { s: Some(full_url.clone()), ..Default::default() });

    let request = GetItemInput {
        table_name: "sauces".to_string(),
        key,
        ..Default::default()
    };
    match client.get_item(request).await {
        Ok(output) => {
            if output.item.is_some() {
                return HttpResponse::Found().insert_header(("Location", full_url.clone())).finish();
            }
            HttpResponse::NotFound().finish()
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}