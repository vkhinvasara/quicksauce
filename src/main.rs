mod models;
mod routes;
mod utils;
mod errors;

use std::sync::{Arc, Mutex};

use actix_web::{App, HttpServer, web};
use routes::AppState;
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient};
use env_logger;
use log::info;



#[actix_web::main]
async fn main() -> std::io::Result<()>{
	dotenv::dotenv().ok();
	env_logger::init();
	info!("Starting server at http://127.0.0.1:8080");
	let client = DynamoDbClient::new(Region::ApSouth1);
	let app_state = web::Data::new(AppState{
		dynamodb_client: client,
	});
    HttpServer::new(move ||{
        App::new()
            .service(routes::create_url)
            .service(routes::redirect)
            .app_data(app_state.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
