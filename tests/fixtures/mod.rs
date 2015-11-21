extern crate rustc_serialize;
extern crate kafka;

use kafka::protocol::*;

pub mod broker;

//We have a message that implements the kafka message traits
#[derive(RustcEncodable, RustcDecodable)]
pub struct MyRequest {
	pub id: i32,
	pub content: String
}

impl ApiMessage for MyRequest { }
impl HasKey for MyRequest {
	fn get_key(&self) -> i32 {
		self.id
	}
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct MyResponse {
	pub id: i32,
	pub content: String
}

impl ApiMessage for MyResponse { }
impl HasKey for MyResponse {
	fn get_key(&self) -> i32 {
		self.id
	}
}

//TODO: Implement separately for each enum variant
#[derive(RustcEncodable, RustcDecodable)]
pub enum MyMessage<T: ApiMessage> {
	Message(T),
	MessageSet(MessageSet<T>)
}

impl <T: ApiMessage> ApiMessage for MyMessage<T> { }