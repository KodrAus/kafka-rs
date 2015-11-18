extern crate kafka;
extern crate stopwatch;

use std::borrow::ToOwned;
use std::sync::Arc;
use std::sync::mpsc;
use std::sync::mpsc::{ Sender, Receiver };
use std::thread;
use std::io::Error;
use std::time::Duration;

use stopwatch::{Stopwatch};

use kafka::protocol::*;
use kafka::connection::*;
use kafka::serialisation::{ serialise, deserialise };

use ::fixtures::*;

const WAIT: u64 = 500;

//A little non-blocking test method that returns a response handle for a request. We just pass in a response factory function to return.
fn request<Req: ApiMessage, Res: ApiMessage>(req: Req, res_factory: fn() -> Res) -> Result<ResponseHandle<Res>, Error> {
	let (tx, rx) = mpsc::channel();
	let res = serialise(&res_factory()).unwrap();

	let _ = thread::spawn(move || {
		thread::sleep(Duration::from_millis(WAIT));

		tx.send(res);
	});
	
	Ok(ResponseHandle::<Res>::new(rx))
}

//A concrete factory for generating a MyResponse that we can pass to our request function above
fn my_response_factory() -> MyResponse {
	MyResponse { 
		id: 1, 
		content: "my request content".to_string() 
	}
}

#[test]
fn can_wrap_serialised_request_in_connection_message() {
	let correlation_id = 42;

	//Create a request message
	let req = MyRequest { 
		id: 1, 
		content: "my request content".to_string() 
	};

	panic!("implement")
}

#[test]
fn can_wrap_serialised_response_in_connection_message() {
	panic!("implement")
}

#[test]
fn can_wrap_serialised_command_in_connection_message() {
	panic!("implement")
}

#[test]
fn can_get_response_from_response_handle_non_blocking() {
	//Generate some response handles, doesn't block the current thread waiting for the response
	let handles = vec![
		request(MyRequest { 
			id: 1, 
			content: "my request content".to_string() 
		}, my_response_factory).unwrap(),
		request(MyRequest { 
			id: 2, 
			content: "my request content".to_string() 
		}, my_response_factory).unwrap(),
		request(MyRequest { 
			id: 3, 
			content: "my request content".to_string() 
		}, my_response_factory).unwrap()
	];

	let sw = Stopwatch::start_new();
	for handle in handles {
		handle.response();
	}

	assert!(sw.elapsed_ms() < (WAIT as i64) * 3);
}