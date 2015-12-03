extern crate bincode;
extern crate kafka;

use kafka::protocol::*;
use kafka::serialisation::{ serialise, deserialise };
use ::fixtures::*;

#[test]
fn can_serialise_and_deserialise_api_requests() {
	//Create a request object
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

	//Serialise the request and then deserialise
	let bytes = serialise(&req).unwrap();
	let des_req = deserialise::<ApiRequestMessage<MyRequest>>(bytes).unwrap();

	assert!(req.request.content == des_req.request.content);
}

#[test]
fn can_serialise_and_deserialise_request_response_message() {
	//Create a request object
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

	//Serialise the message
	let msg = RequestResponseMessage::new(&req);
	let bytes = serialise(&msg).unwrap();

	//Deserialise the message
	match deserialise::<RequestResponseMessage>(bytes) {
		Ok(_msg) => assert!(msg.size == _msg.size),
		Err(e) => panic!("{:}", e)
	}
}