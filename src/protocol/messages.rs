extern crate rustc_serialize;

use std::any::Any;
use rustc_serialize::{ Encodable, Decodable, Encoder, Decoder };
use ::serialisation::*;

/// A standard Kafka message format for both requests and responses in protocol APIs to implement.
/// The ApiMessage type is also the right trait for you to implement on your custom event types.
pub trait ApiMessage: Encodable + Decodable + Any { }

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

//TODO: Move this logic into the serialisation module
impl Encodable for RequestResponseMessage {
	fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
		//Emit the length as 4 bytes
		s.emit_u32(self.size);

		//Emit the contents of the byte array
		for (i, e) in self.bytes.iter().enumerate() {
            try!(s.emit_seq_elt(i, |s| s.emit_u8(*e)));
        }

        Ok(())
	}
}

impl Decodable for RequestResponseMessage {
	fn decode<D: Decoder>(d: &mut D) -> Result<RequestResponseMessage, D::Error> {
		//TODO: Make this work
        d.read_seq(|d, len| {
        	//Decode the vec
            let mut v = Vec::with_capacity(len);
            for i in 0..len {
                v.push(try!(d.read_seq_elt(i, |d| Decodable::decode(d))));
            }

            Ok(RequestResponseMessage {
            	size: len as u32,
            	bytes: v
            })
        })
    }
}