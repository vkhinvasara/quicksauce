use url::Url;
use uuid::Uuid;

pub struct Sauce{
	pub id: Uuid,
	pub url: Url,
	pub short_url: Url,
}



impl Sauce {
	pub fn new(url: String, short_url: String) -> Self {
		Self {
			id: Uuid::new_v4(),
			url: Url::parse(&url).expect("Invalid URL provided!").to_string().parse().unwrap(),
			short_url: Url::parse(&short_url).expect("Trouble parsing short URL!").to_string().parse().unwrap(),
		}
	}
}