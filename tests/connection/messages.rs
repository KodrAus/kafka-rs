extern crate kafka;

use std::sync::mpsc;
use kafka::conn::*;
use kafka::encoding::{ encode };

use ::fixtures::*;

#[test]
fn can_wrap_serialised_request_in_connection_message() {
	//Create a request message
	let req = MyRequest { 
		id: 1, 
		content: "my request content".to_string() 
	};

	//Serialise and wrap in a connection message
	//This takes a sender to asynchronously send the response on
	let (tx, _) = mpsc::channel();
	let _ = ConnectionMessage::Request(tx, encode(&req).unwrap());
}

#[test]
fn can_wrap_serialised_response_in_connection_message() {
	//Create a response message
	let res = MyResponse { 
		id: 1, 
		content: "my response content".to_string() 
	};

	//Serialise and wrap in a connection message
	let _ = ConnectionMessage::Response(encode(&res).unwrap());
}

#[test]
fn can_wrap_serialised_command_in_connection_message() {
	//Create a request message
	let req = MyRequest { 
		id: 1, 
		content: "my request content".to_string() 
	};

	//Serialise and wrap in a connection message
	let _ = ConnectionMessage::Execute(encode(&req).unwrap());
}