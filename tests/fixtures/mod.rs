extern crate kafka;

use std::io::Error;

use kafka::serialisation::*;
use kafka::protocol::*;

//We have a message that implements the kafka message traits
pub struct MyRequest {
	pub id: i32,
	pub content: String
}
impl Message for MyRequest {
	fn get_key(&self) -> i32 {
		self.id
	}
}
impl ToBytes for MyRequest {
	fn to_bytes(&self) -> &[u8] {
		&[]
	}
}
impl FromBytes for MyRequest {
	type Deserialised = MyRequest;

	fn from_bytes(bytes: &[u8]) -> Result<MyRequest, Error> {
		Ok(MyRequest {
			id: 1,
			content: "my deserialised content".to_string()
		})
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
impl ToBytes for MyResponse {
	fn to_bytes(&self) -> &[u8] {
		&[]
	}
}
impl FromBytes for MyResponse {
	type Deserialised = MyResponse;

	fn from_bytes(bytes: &[u8]) -> Result<MyResponse, Error> {
		Ok(MyResponse {
			id: 1,
			content: "my deserialised content".to_string()
		})
	}
}