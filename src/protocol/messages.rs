use std::io::{ Error, ErrorKind };

use ::serialisation::*;

/// A standard Kafka message format for both requests and responses in protocol APIs to implement.
/// The ApiMessage type is also the right trait for you to implement on your custom event types.
pub trait ApiMessage: ToBytes + FromBytes {
	fn get_key(&self) -> i32;
	fn new() -> Self where Self: Sized;
}

/// The standard structure of all Kafka requests. API-specific detail is provided by the request parameter
pub struct ApiRequestMessage<T: ApiMessage> {
	pub api_key: i16,
	pub api_version: i16,
	pub correlation_id: i32,
	pub client_id: String,
	pub request: T
}

impl <T: ApiMessage> ApiRequestMessage<T> {
	fn new() -> ApiRequestMessage<T> {
		ApiRequestMessage {
			api_key: 1,
			api_version: 1,
			correlation_id: 1,
			client_id: "".to_string(),
			request: T::new()
		}
	}
}

impl <T: ApiMessage> ToBytes for ApiRequestMessage<T> {
	fn to_bytes(&self) -> &[u8] {
		//TODO: Implement
		&[]
	}
}

impl <T: ApiMessage> FromBytes for ApiRequestMessage<T> {
	fn from_bytes(bytes: &[u8]) -> Result<ApiRequestMessage<T>, Error> {
		let mut req = ApiRequestMessage::<T>::new();

		//Deserialise the ApiRequestMessage component
		//Deserialise the T component

		//TODO: Implement
		Ok(req)
	}
}

/// The standard structure of all Kafka responses. API-specific detail is provided by the response parameter
pub struct ApiResponseMessage<T: ApiMessage> {
	pub correlation_id: i32,
	pub response: T
}

impl <T: ApiMessage> ApiResponseMessage<T> {
	fn new() -> ApiResponseMessage<T> {
		ApiResponseMessage {
			correlation_id: 1,
			response: T::new()
		}
	}
}

impl <T: ApiMessage> ToBytes for ApiResponseMessage<T> {
	fn to_bytes(&self) -> &[u8] {
		//TODO: Implement
		&[]
	}
}

impl <T: ApiMessage> FromBytes for ApiResponseMessage<T> {
	fn from_bytes(bytes: &[u8]) -> Result<ApiResponseMessage<T>, Error> {
		let mut res = ApiResponseMessage::<T>::new();

		//Deserialise the ApiResponseMessage component
		//Deserialise the T component

		//TODO: Implement
		Ok(res)
	}
}

//TODO: Hide concerns that aren't necessary for the user to worry about
pub struct Message<T: ApiMessage> {
	pub crc: i32,
	pub magic_byte: i8,
	pub compression: Compression,
	pub key: String,
	pub value: T
}

/// A standard collection of kafka messages as a sequence of key-value pairs
pub struct MessageSet<T: ApiMessage> {
	pub offset: i64,
	pub message_size: i32,
	pub messages: Vec<Message<T>>
}

impl <T: ApiMessage> ApiMessage for MessageSet<T> {
	fn get_key(&self) -> i32 { 0 }

	fn new() -> MessageSet<T> {
		MessageSet::<T> {
			offset: 0,
			message_size: 0,
			messages: Vec::new()
		}
	}
}

impl <T: ApiMessage> ToBytes for MessageSet<T> {
	fn to_bytes(&self) -> &[u8] {
		//TODO: Implement
		&[]
	}
}

impl <T: ApiMessage> FromBytes for MessageSet<T> {
	fn from_bytes(bytes: &[u8]) -> Result<MessageSet<T>, Error> {
		let mut res = MessageSet::<T>::new();

		//Deserialise the MessageSet component
		//Deserialise the Messages components

		//TODO: Implement
		Ok(res)
	}
}