extern crate kafka;

use kafka::protocol::*;

//We have a message that implements the kafka message trait
pub struct MyRequest {
	pub id: i32,
	pub content: String
}
impl Message for MyRequest {
	fn get_key(&self) -> i32 {
		self.id
	}
}

pub struct MyResponse {
	pub id: i32,
	pub content: String
}
impl Message for MyResponse {
	fn get_key(&self) -> i32 {
		self.id
	}
}