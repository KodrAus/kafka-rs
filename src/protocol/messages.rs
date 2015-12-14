extern crate rustc_serialize;

use std::any::Any;
use rustc_serialize::{ Encodable, Decodable };

/// A standard Kafka message format for both requests and responses in protocol APIs to implement.
/// The ApiMessage type is also the right trait for you to implement on your custom event types.
pub trait ApiMessage: Encodable + Decodable + Any + Clone { }

pub trait HasKey {
	fn get_key(&self) -> i32;
}

/// The standard structure of all Kafka requests. API-specific detail is provided by the request parameter
#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct ApiRequestMessage<T: ApiMessage> {
	pub api_key: i16,
	pub api_version: i16,
	pub correlation_id: i32,
	pub client_id: String,
	pub request: T
}

impl <T: ApiMessage> ApiMessage for ApiRequestMessage<T> { }

/// The standard structure of all Kafka responses. API-specific detail is provided by the response parameter
#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct ApiResponseMessage<T: ApiMessage> {
	pub correlation_id: i32,
	pub response: T
}

impl <T: ApiMessage> ApiMessage for ApiResponseMessage<T> { }

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct Message<T: ApiMessage> {
	pub crc: i32,
	pub magic_byte: i8,
	pub compression: Compression,
	pub key: String,
	pub value: T
}

/// A standard collection of kafka messages as a sequence of key-value pairs
#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct MessageSet<T: ApiMessage> {
	pub offset: i64,
	pub message_size: i32,
	pub messages: Vec<Message<T>>
}

impl <T: ApiMessage> ApiMessage for MessageSet<T> { }

#[derive(RustcEncodable, RustcDecodable, Clone)]
pub enum Compression {
	None,
	Gzip,
	Snappy
}