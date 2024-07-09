use base64::{engine::general_purpose::STANDARD, Engine};
use sha2::{Sha256, Digest};
use dotenv::dotenv;

/// Shortens a URL by hashing it and encoding the hash in base64.
/// Returns the first 5 characters of the base64-encoded hash.
/// Note: This method increases the risk of hash collisions.


const SHORT_URL_LENGTH: usize = 5;

pub fn shorten_url(url: &str) -> String {
	let mut hasher = Sha256::new();
	hasher.update(url);
	let result = hasher.finalize();
	let mut encoded = String::new();

	STANDARD.encode_string(&result, &mut encoded);
	let code = encoded[0..SHORT_URL_LENGTH].to_string();
	dotenv().ok();
	dotenv::var("BASE_URL").unwrap() + &code
}