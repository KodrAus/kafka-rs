use std::io::{ Error, ErrorKind };

use ::serialisation::*;

//TODO: May need to be more general with ApiRequestMessage and ApiResponseMessage for serialisation
//Possibly split into separate SingleHeader and MultiHeader
//Standard request/response type
pub trait Message: Sized + Send + ToBytes + FromBytes {
	fn get_key(&self) -> i32;
}

//Standard Kafka Request
pub struct ApiRequestMessage<T: Message> {
	pub api_key: i16,
	pub api_version: i16,
	pub correlation_id: i32,
	pub client_id: String,
	pub request: T
}
impl <T: Message> ToBytes for ApiRequestMessage<T> {
	fn to_bytes(&self) -> &[u8] {
		&[]
	}
}
impl <T: Message> FromBytes for ApiRequestMessage<T> {
	type Deserialised = ApiRequestMessage<T>;

	fn from_bytes(bytes: &[u8]) -> Result<ApiRequestMessage<T>, Error> {
		Err(Error::new(
			ErrorKind::Other, 
			"implement"
		))
	}
}

//Standard Kafka Response
pub struct ApiResponseMessage<T: Message> {
	pub correlation_id: i32,
	pub response: T
}
impl <T: Message> ToBytes for ApiResponseMessage<T> {
	fn to_bytes(&self) -> &[u8] {
		&[]
	}
}
impl <T: Message> FromBytes for ApiResponseMessage<T> {
	type Deserialised = ApiResponseMessage<T>;

	fn from_bytes(bytes: &[u8]) -> Result<ApiResponseMessage<T>, Error> {
		Err(Error::new(
			ErrorKind::Other, 
			"implement"
		))
	}
}
