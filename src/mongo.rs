use mongodb::{
	// error::Result,
	Client,
	options::ClientOptions,
};
use crate::block::Block;

pub struct ConnectionInfo<'a> {
	pub url_str: &'a str,
	pub app_name: &'a str,
	pub db_name: &'a str,
	pub cll_name: &'a str,
}

impl<'a> ConnectionInfo<'a> {
	pub fn new() -> ConnectionInfo<'a> {
		ConnectionInfo {
			url_str: 	"mongodb://localhost:27017",
			app_name:	"collect recipe program",
			db_name: 	"test",						// "chef_recipes"
			cll_name: 	"test_coll"			// "structs"
		}
	}

	// get a handle to the "structs" collection
	pub async fn insert_structual_data(&self, blocks: Vec<Block>) {
		let mut client_options = ClientOptions::parse(self.url_str).await.unwrap();
		client_options.app_name = Some(self.app_name.to_string());
		let client = Client::with_options(client_options).unwrap();

		let db = client.database(self.db_name);
		let collection = db.collection::<Block>(self.cll_name);

		collection.insert_many(blocks, None).await.unwrap();
	}
}
