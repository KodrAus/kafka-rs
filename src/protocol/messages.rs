extern crate rustc_serialize;

use rustc_serialize::{ Encodable, Decodable };

use ::serialisation::*;

/// A standard Kafka message format for both requests and responses in protocol APIs to implement.
/// The ApiMessage type is also the right trait for you to implement on your custom event types.
pub trait ApiMessage: Encodable + Decodable { }

pub trait HasKey {
	fn get_key(&self) -> i32;
}

/// The standard structure of all Kafka requests. API-specific detail is provided by the request parameter
#[derive(RustcEncodable, RustcDecodable)]
pub struct ApiRequestMessage<T: ApiMessage> {
	pub api_key: i16,
	pub api_version: i16,
	pub correlation_id: i32,
	pub client_id: String,
	pub request: T
}

impl <T: ApiMessage> ApiMessage for ApiRequestMessage<T> { }

/// The standard structure of all Kafka responses. API-specific detail is provided by the response parameter
#[derive(RustcEncodable, RustcDecodable)]
pub struct ApiResponseMessage<T: ApiMessage> {
	pub correlation_id: i32,
	pub response: T
}

impl <T: ApiMessage> ApiMessage for ApiResponseMessage<T> { }

//TODO: Hide concerns that aren't necessary for the user to worry about
#[derive(RustcEncodable, RustcDecodable)]
pub struct Message<T: ApiMessage> {
	pub crc: i32,
	pub magic_byte: i8,
	pub compression: Compression,
	pub key: String,
	pub value: T
}

/// A standard collection of kafka messages as a sequence of key-value pairs
#[derive(RustcEncodable, RustcDecodable)]
pub struct MessageSet<T: ApiMessage> {
	pub offset: i64,
	pub message_size: i32,
	pub messages: Vec<Message<T>>
}

impl <T: ApiMessage> ApiMessage for MessageSet<T> { }

pub struct RequestResponseMessage {
	pub size: u32,
	pub bytes: Vec<u8>
}

impl RequestResponseMessage {
	pub fn new<T: ApiMessage>(msg: &T) -> RequestResponseMessage {
		let bytes = match serialise(&msg) {
			Ok(b) => b,
			Err(e) => panic!("failed to serialise message")
		};

		RequestResponseMessage {
			size: bytes.len() as u32,
			bytes: bytes
		}
	}
}