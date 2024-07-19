use crate::models::Sauce;
use crate::utils::shorten_url;
use actix_web::{get, post, web, HttpResponse, Responder};
use dotenv::dotenv;
use rusoto_core::RusotoError;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, PutItemError, PutItemInput};
use serde::Deserialize;
use std::collections::HashMap;
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
    let (short_url, id) = shorten_url(&url);
    let item = Sauce::new(id.clone(),url.to_string(), short_url.to_string());
    let mut attribute_values = HashMap::new();
    attribute_values.insert(
        "id".to_string(),
        AttributeValue {
            s: Some(item.id.to_string()),
            ..Default::default()
        },
    );
    attribute_values.insert(
        "url".to_string(),
        AttributeValue {
            s: Some(item.url.to_string()),
            ..Default::default()
        },
    );
    attribute_values.insert(
        "short_url".to_string(),
        AttributeValue {
            s: Some(item.short_url.to_string()),
            ..Default::default()
        },
    );
    let input = PutItemInput {
        table_name: "sauces".to_string(),
        item: attribute_values,
        condition_expression: Some("attribute_not_exists(id)".to_string()),
        ..Default::default()
    };
    match client.put_item(input).await {
        Ok(_) => HttpResponse::Ok().body(short_url.clone()),
        Err(error) => match error{
            RusotoError::Service(PutItemError::ConditionalCheckFailed(error_msg)) =>{ 
                let mut key = HashMap::new();
                key.insert(
                    "id".to_string(),
                    AttributeValue {
                        s: Some(id.clone()),
                        ..Default::default()
                    },
                );
                let request = GetItemInput {
                    table_name: "sauces".to_string(),
                    key,
                    ..Default::default()
                };
                match client.get_item(request).await {
                    Ok(output) => {
                        if output.item.is_some() {
                            return HttpResponse::Ok().body(output.item.unwrap()["short_url"].s.clone().unwrap());
                        }
                        HttpResponse::NotFound().finish()
                    }
                    Err(err) => {
                        HttpResponse::InternalServerError().body(format!("Error fetching short URL: {:?}", err))
                    },
                }
            },
        _=> HttpResponse::InternalServerError().body(format!("Error creating short URL: {:?}", error)),
        }
    }
}

#[get("/{id}")]
async fn redirect(id: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    dotenv().ok(); // Load environment variables
    // let base_url = env::var("BASE_URL").expect("BASE_URL must be set in .env file");
    // let full_url = format!("{}{}", base_url, id.into_inner()); // Construct the full URL
    let id = id.into_inner();
    let client = &data.dynamodb_client;
    let mut key = HashMap::new();
    key.insert(
        "id".to_string(),
        AttributeValue {
            s: Some(id.clone()),
            ..Default::default()
        },
    );

    let request = GetItemInput {
        table_name: "sauces".to_string(),
        key,
        ..Default::default()
    };

    match client.get_item(request).await {
        Ok(output) => {
            if output.item.is_some() {
                return HttpResponse::Found()
                    .insert_header(("Location", output.item.unwrap()["url"].s.clone().unwrap()))
                    .finish();
            }
            HttpResponse::NotFound().finish()
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Error fetching short URL: {:?}", err))
        },
    }
}
