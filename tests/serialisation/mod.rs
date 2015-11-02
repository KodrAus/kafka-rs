extern crate kafka;

use std::io::Error;

use kafka::serialisation::*;
use kafka::protocol::*;

use ::fixtures::*;

//Generic methods to serialise and deserialise messages without knowing their exact type
fn serialise<T: ToBytes>(data: &T) -> &[u8] {
	data.to_bytes()
}
fn deserialise<T: FromBytes>(bytes: &[u8]) -> Result<<T as FromBytes>::Deserialised, Error> {
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