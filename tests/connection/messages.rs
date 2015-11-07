extern crate kafka;

use std::sync::Arc;
use std::sync::mpsc;
use std::thread;
use std::io::Error;

use kafka::protocol::*;
use kafka::connection::ConnectionMessage;

use ::fixtures::*;

//Receives a request message and sends a response message with the same correlation id
//This could use the generic serialisation methods, but for this test it doesn't matter
fn respond(req: ConnectionMessage<MyRequest, MyResponse>) -> Result<ConnectionMessage<MyRequest, MyResponse>, Error> {
	match req {
		ConnectionMessage::Request::<MyRequest, MyResponse>(tx, req) => {
			let res = Arc::new(ApiResponseMessage { 
				response: MyResponse { 
					id: req.request.id,
					content: "my response content".to_string() 
				},
				correlation_id: req.correlation_id
			});

			tx.send(res.clone()).unwrap();

			Ok(ConnectionMessage::Response(res.clone()))
		},
		_ => panic!("unexpected connection message. Was waiting for a ConnectionMessage::Request")
	}
}

#[test]
fn can_wrap_request_and_response_in_connection_message() {
	let correlation_id = 42;

	//Create a request message
	let req = ApiRequestMessage { 
		request: MyRequest { 
			id: 1, 
			content: "my request content".to_string() 
		}, 
		api_key: 1,
		api_version: 1,
		correlation_id: correlation_id,
		client_id: "some id".to_string()
	};

	//Create a channel to send and receive the response on
	let (tx, rx) = mpsc::channel();

	//Create a request, and pass in the channel
	//This takes ownership of the request, which is no longer 'valid' once it's been sent
	let handle = thread::spawn(move || {
		respond(ConnectionMessage::Request(tx, req)).unwrap();
	});

	//Wait for a response to be received
	let res = rx.recv().unwrap();
	handle.join().unwrap();

	assert!(res.correlation_id == correlation_id);
}
