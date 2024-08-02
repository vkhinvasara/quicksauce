use std::collections::HashMap;

use base64::{engine::general_purpose::STANDARD, Engine};
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput};
use sha2::{Sha256, Digest};
use dotenv::dotenv;

const SHORT_URL_LENGTH: usize = 5;

pub async fn shorten_url(url: &str) -> (String, String) {
    let mut hasher = Sha256::new();
    hasher.update(url);
    let result = hasher.finalize();
    let mut encoded = String::new();

    STANDARD.encode_string(&result, &mut encoded);

    let mut start = 0;
    let mut end = SHORT_URL_LENGTH;
    let mut code = encoded[start..end].to_string();

    while is_collision(&code).await{
        if end >= encoded.len() {
            break;
        }
        start += 1;
        end += 1;
        code = encoded[start..end].to_string();
    }

    dotenv().ok();
    (dotenv::var("BASE_URL").unwrap() + &code, code)
}

async fn is_collision(code: &str) -> bool {
	let client = DynamoDbClient::new(Region::default());
	let input = GetItemInput {
		table_name: "sauces".to_string(),
		key: {
			let mut key = HashMap::new();
			key.insert(
				"id".to_string(),
				AttributeValue {
					s: Some(code.to_string()),
					..Default::default()
				},
			);
			key
		},
		..Default::default()
	};
	match client.get_item(input).await{
		Ok(_) => true,
		Err(_) => false,
	}
}