//Other protocol modules implement their own Message types, that all wrap within the same ApiRequestMessage and ApiResponseMessage

pub trait Message: Sized + Send {
	fn get_key(&self) -> i32;
}

pub struct ApiRequestMessage<T: Message> {
	pub api_key: i16,
	pub api_version: i16,
	pub correlation_id: i32,
	pub client_id: String,
	pub request: T
}

pub struct ApiResponseMessage<T: Message> {
	pub correlation_id: i32,
	pub response: T
}