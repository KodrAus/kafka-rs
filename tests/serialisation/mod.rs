extern crate kafka;

use std::io::Error;

use kafka::serialisation::*;
use kafka::protocol::*;

use ::fixtures::*;

//Generic methods to serialise and deserialise messages without knowing their exact type
fn serialise<T: ToBytes>(data: &T) -> &[u8] {
	data.to_bytes()
}
fn deserialise<T: FromBytes>(bytes: &[u8]) -> Result<T, Error> {
	T::from_bytes(bytes)
}

#[test]
fn can_call_to_bytes_on_generic_type() {
	//Create a request
	let msg = MyRequest {
		id: 1,
		content: "some content".to_string()
	};

	//Serialise the request
	let _ = serialise(&msg);
}

#[test]
fn can_call_from_bytes_on_generic_type() {
	//Create some bytes (no bytes 4 u)
	let bytes: [u8; 0] = [];

	//Deserialise to a response
	let _ = deserialise::<MyResponse>(&bytes);
}

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
	let bytes = req.to_bytes();
	let des_req = ApiRequestMessage::<MyRequest>::from_bytes(&bytes).unwrap();

	assert!(req.request.content == des_req.request.content);
}