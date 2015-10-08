pub mod fetch;
pub mod metadata;
pub mod offset_commit;
pub mod offset_fetch;
pub mod offsets;
pub mod send;

//TODO: Implement protocols. Each protocol has its own kind of self-contained request/response implementations

//What is a message? It is a protocol for the kind of things sent, a container for a message body

//Message traits for api modules to implement
//TODO: Should go in its own mod, not the base protocol module
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