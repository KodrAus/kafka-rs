extern crate bincode;
extern crate kafka;

use std::io::Error;

use bincode::rustc_serialize::{ encode, decode };

use kafka::serialisation::*;
use kafka::protocol::*;

use ::fixtures::*;

//Generic methods to serialise and deserialise messages without knowing their exact type
//TODO: Get these to compile using bincode serialisation
fn serialise<T: ToBytes>(data: &T) -> &[u8] {
	encode(data).unwrap()
}
fn deserialise<T: FromBytes>(bytes: &[u8]) -> Result<T, Error> {
	decode(bytes)
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
	let bytes = serialise(&req);
	let des_req = deserialise::<ApiRequestMessage::<MyRequest>>(&bytes).unwrap();

	assert!(req.request.content == des_req.request.content);
}
