extern crate kafka;
extern crate stopwatch;

use std::sync::mpsc;
use std::thread;
use std::io::Error;
use std::time::Duration;
use stopwatch::{ Stopwatch };
use kafka::client::ResponseHandle;
use kafka::client::protocol::*;
use kafka::conn::*;
use kafka::encoding::encode;
use ::fixtures::*;

const WAIT: u64 = 500;

//A little non-blocking test method that returns a response handle for a request. We just pass in a response factory function to return.
fn request<Req: ApiMessage, Res: ApiMessage>(_req: Req, res_factory: fn() -> Res) -> Result<ResponseHandle<Res>, Error> {	
	let (tx, rx) = mpsc::channel();
	let res = encode(&res_factory()).unwrap();

	let _ = thread::spawn(move || {
		thread::sleep(Duration::from_millis(WAIT));

		let _ = tx.send(res);
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
fn can_get_response_from_response_handle_non_blocking() {
	//Generate some response handles, doesn't block the current thread waiting for the response
	let mut handles = (1..3).map(|i| {
		request(MyRequest { 
			id: i, 
			content: "my request content".to_string() 
		}, my_response_factory).unwrap()
	}).collect::<Vec<ResponseHandle<MyResponse>>>();

	let sw = Stopwatch::start_new();

	//Wait for the response from each handle. They should wait concurrently, so the total time is roughly equal to waiting on a single handle
	for handle in &mut handles {
		handle.response();
	}

	assert!(sw.elapsed_ms() < (WAIT as i64) * 3);
}

#[test]
fn response_handle_does_not_block_more_than_once() {
	//Generate some response handles, doesn't block the current thread waiting for the response
	let mut handle = request(MyRequest { 
		id: 1, 
		content: "my request content".to_string() 
	}, my_response_factory).unwrap();

	let sw = Stopwatch::start_new();

	//Try to get the same response from the handle more than once
	//Only the first call should block
	handle.response();
	handle.response();

	assert!(sw.elapsed_ms() < (WAIT as i64) * 2);
}

#[test]
fn response_handle_can_block_for_n_messages() {
	panic!("implement")
}

#[test]
fn response_handle_can_block_for_infinite_messages() {
	panic!("implement")
}