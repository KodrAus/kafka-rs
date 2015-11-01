extern crate kafka;

use kafka::protocol::*;

use ::fixtures::*;

#[test]
fn can_get_message_from_request() {
	let id = 1;
	let msg = MyRequest { 
		id: id, 
		content: "my request content".to_string() 
	};

	let req = ApiRequestMessage { 
		request: msg, 
		api_key: 1,
		api_version: 1,
		correlation_id: 1,
		client_id: "some id".to_string()
	};

	let req_msg = req.request;

	assert!(id == req_msg.id)
}

#[test]
fn can_get_message_from_response() {
	let id = 1;
	let msg = MyResponse {
		id: id,
		content: "my response content".to_string()
	};

	let res = ApiResponseMessage {
		response: msg,
		correlation_id: 1
	};

	let res_msg = res.response;

	assert!(id == res_msg.id);
}