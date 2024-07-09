use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemInput, GetItemInput};
use actix_web::{post, web, HttpResponse, Responder, get};
use crate::utils::shorten_url;
use crate::models::Sauce;
use std::collections::HashMap;

struct AppState {
    dynamodb_client: DynamoDbClient,
}

#[post("/create_url")]
async fn create_url(url: web::Json<String>, data: web::Data<AppState>) -> impl Responder {
    let url = url.into_inner();
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

#[get("/{short_url}")]
async fn redirect(short_url: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let short_url = short_url.into_inner();
    let client = &data.dynamodb_client;

    let mut key = HashMap::new();
    key.insert("short_url".to_string(), AttributeValue { s: Some(short_url), ..Default::default() });

    let request = GetItemInput {
        table_name: "YourTableName".to_string(),
        key,
        ..Default::default()
    };
    match client.get_item(request).await {
        Ok(output) => {
            if let Some(item) = output.item {
                if let Some(attr) = item.get("full_url") {
                    if let Some(full_url) = &attr.s {
                        return HttpResponse::Found().insert_header(("Location", full_url.to_string())).finish();
                    }
                }
            }
            HttpResponse::NotFound().finish()
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}