pub trait Message: Sized + Send {
	fn get_key(&self) -> i32;
}

pub struct ApiRequest {
	pub api_key: i16,
	pub api_version: i16,
	pub correlation_id: i32,
	pub client_id: String
}

pub trait ApiRequestMessage<T: Message> {
	fn get_request(&self) -> &ApiRequest;
	fn get_message(&self) -> &T;
}

pub trait ApiResponseMessage<T: Message> {
	fn get_response(&self) -> &T;
}