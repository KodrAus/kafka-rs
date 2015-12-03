extern crate kafka;

use std::sync::mpsc;
use kafka::connection::*;
use kafka::serialisation::{ serialise };

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
	let _ = BufferedConnectionMessage::Request(tx, serialise(&req).unwrap());
}

#[test]
fn can_wrap_serialised_response_in_connection_message() {
	//Create a response message
	let res = MyResponse { 
		id: 1, 
		content: "my response content".to_string() 
	};

	//Serialise and wrap in a connection message
	let _ = BufferedConnectionMessage::Response(serialise(&res).unwrap());
}

#[test]
fn can_wrap_serialised_command_in_connection_message() {
	//Create a request message
	let req = MyRequest { 
		id: 1, 
		content: "my request content".to_string() 
	};

	//Serialise and wrap in a connection message
	let _ = BufferedConnectionMessage::Execute(serialise(&req).unwrap());
}