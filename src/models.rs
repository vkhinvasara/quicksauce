use url::Url;

pub struct Sauce{
	pub id: String,
	pub url: Url,
	pub short_url: Url,
}
impl Sauce {
	pub fn new(id:String, url: String, short_url: String) -> Self {
		Self {
			id,
			url: Url::parse(&url).expect("Invalid URL provided!").to_string().parse().unwrap(),
			short_url: Url::parse(&short_url).expect("Trouble parsing short URL!").to_string().parse().unwrap(),
		}
	}
}