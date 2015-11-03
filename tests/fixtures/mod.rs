extern crate kafka;

use std::io::Error;

use kafka::serialisation::*;
use kafka::protocol::*;

//We have a message that implements the kafka message traits
pub struct MyRequest {
	pub id: i32,
	pub content: String
}

impl ApiMessage for MyRequest {
	fn get_key(&self) -> i32 {
		self.id
	}

	fn new() -> MyRequest {
		MyRequest {
			id: 1,
			content: "".to_string()
		}
	}
}

impl ToBytes for MyRequest {
	fn to_bytes(&self) -> &[u8] {
		&[]
	}
}

impl FromBytes for MyRequest {
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

impl ApiMessage for MyResponse {
	fn get_key(&self) -> i32 {
		self.id
	}

	fn new() -> MyResponse {
		MyResponse {
			id: 1,
			content: "".to_string()
		}
	}
}

impl ToBytes for MyResponse {
	fn to_bytes(&self) -> &[u8] {
		&[]
	}
}

impl FromBytes for MyResponse {
	fn from_bytes(bytes: &[u8]) -> Result<MyResponse, Error> {
		Ok(MyResponse {
			id: 1,
			content: "my deserialised content".to_string()
		})
	}
}

//TODO: Implement separately for each enum variant
pub enum MyMessage<T: ApiMessage> {
	Message(T),
	MessageSet(MessageSet<T>)
}

impl <T: ApiMessage> ApiMessage for MyMessage<T> {
	fn get_key(&self) -> i32 { 0 }

	fn new() -> MyMessage<T> {
		MyMessage::Message(T::new())
	}
}

impl <T: ApiMessage> ToBytes for MyMessage<T> {
	fn to_bytes(&self) -> &[u8] {
		&[]
	}
}

impl <T: ApiMessage> FromBytes for MyMessage<T> {
	fn from_bytes(bytes: &[u8]) -> Result<MyMessage<T>, Error> {
		Ok(MyMessage::Message(T::new()))
	}
}