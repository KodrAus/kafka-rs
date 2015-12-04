extern crate bincode;
extern crate kafka;

use kafka::client::protocol::*;
use kafka::encoding::{ encode, decode };
use ::fixtures::*;

#[test]
fn can_encode_and_deode_api_requests() {
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
	let bytes = encode(&req).unwrap();
	let des_req = decode::<ApiRequestMessage<MyRequest>>(bytes).unwrap();

	assert!(req.request.content == des_req.request.content);
}

#[test]
fn can_preppend_and_get_message_length() {
	panic!("implement")
}