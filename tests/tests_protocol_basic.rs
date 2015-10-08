extern crate kafka;

use kafka::protocol;
use kafka::protocol::{ Message, ApiRequestMessage, ApiResponseMessage, ApiRequest };

//We have a message that implements the kafka message trait
struct MyMessage {
	id: i32,
	content: String
}
impl Message for MyMessage {
	fn get_key(&self) -> i32 {
		self.id
	}
}

//We have a request message type for our custom kafka api module
//An api module will provide its own implementations off ApiRequestMessages to keep things encapsulated
struct MyApiRequestMessage<T: Message> {
	pub req_data: ApiRequest,
	pub req_msg: T
}
impl<T: Message> ApiRequestMessage<T> for MyApiRequestMessage<T> {
	fn get_request(&self) -> &ApiRequest {
		&self.req_data
	}

	fn get_message(&self) -> &T {
		&self.req_msg
	}
}

//We also provide an api response for the same reasons
struct MyApiResponseMessage<T: Message> {
	pub res_msg: T
}
impl<T: Message> ApiResponseMessage<T> for MyApiResponseMessage<T> {
	fn get_response(&self) -> &T {
		&self.res_msg
	}
}

#[test]
fn can_get_message_from_request() {
	let id = 1;
	let msg = MyMessage { 
		id: id, 
		content: "my content".to_string() 
	};

	//TODO: Make this a nicer builder
	//Takes ownership of the message
	let req = MyApiRequestMessage { 
		req_msg: msg, 
		req_data: ApiRequest {
			api_key: 1,
			api_version: 1,
			correlation_id: 1,
			client_id: "some id".to_string()
		} 
	};

	//Gets an immutable borrow of the message content
	let req_msg = req.get_message();

	assert!(id == req_msg.id)
}