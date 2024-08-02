mod models;
mod routes;
mod utils;
mod errors;


use actix_web::{App, HttpServer, web};
use routes::AppState;
use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;
use env_logger;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    info!("Starting server at http://localhost:8080");
    let client = DynamoDbClient::new(Region::default());
    let app_state = web::Data::new(AppState {
        dynamodb_client: client,
    });
    HttpServer::new(move || {
        App::new()
            .service(routes::create_url)
            .service(routes::redirect)
            .app_data(app_state.clone())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}