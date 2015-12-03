extern crate kafka;

use kafka::protocol::*;
use kafka::serialisation::*;
use ::fixtures::*;

#[test]
fn can_get_message_from_api_request() {
	let id = 1;

	let req = ApiRequestMessage { 
		request: MyRequest { 
			id: id, 
			content: "my request content".to_string() 
		}, 
		api_key: 1,
		api_version: 1,
		correlation_id: 1,
		client_id: "some id".to_string()
	};

	let req_msg = req.request;

	assert!(id == req_msg.id)
}

#[test]
fn can_get_message_from_api_response() {
	let id = 1;

	let res = ApiResponseMessage {
		response: MyResponse {
			id: id,
			content: "my response content".to_string()
		},
		correlation_id: 1
	};

	let res_msg = res.response;

	assert!(id == res_msg.id);
}

#[test]
fn message_set_should_take_message_sets_in_vec() {
	//TODO: Build a macro for this
	//Shows how we can use an enum to wrap our message types and support multiple messages
	let _ = MessageSet::<MyMessage<MyRequest>> {
		offset: 0,
		message_size: 0,
		messages: vec![ 
			//Message 1
			Message {
				crc: 0,
				magic_byte: 0,
				compression: Compression::None,
				key: "".to_string(),
				//This message contains a MyRequest
				value: MyMessage::Message(
					MyRequest {
						id: 1, 
						content: "my request content".to_string() 
					}
				)
			},
			//Message 2
			Message {
				crc: 0,
				magic_byte: 0,
				compression: Compression::None,
				key: "".to_string(),
				//This message contains another MessageSet
				value: MyMessage::MessageSet( 
					MessageSet {
						offset: 0,
						message_size: 0,
						messages: Vec::new()
					}
				)
			}
		]
	};
}

#[test]
fn request_response_message_contains_message_size() {
	//Create a request
	let req = ApiRequestMessage { 
		request: MyRequest { 
			id: 1, 
			content: "my request content".to_string() 
		}, 
		api_key: 1,
		api_version: 1,
		correlation_id: 1,
		client_id: "some id".to_string()
	};

	//Wrap the request in the standard req/res format
	let msg = RequestResponseMessage::new(&req);

	assert!(msg.size > 0);
}