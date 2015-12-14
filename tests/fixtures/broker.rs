extern crate mio;
extern crate kafka;

use kafka::protocol::ApiMessage;
use kafka::protocol::encoding::{ encode };
use kafka::sync::{ Sender };
use kafka::conn::ConnectionHandler;

pub struct TestHandler<T: ApiMessage> {
	res: T,
	handled: u16
}

impl <T: ApiMessage> TestHandler<T> {
	fn get_res(&self) -> &T {
		&self.res
	}

	fn new(res: T) -> TestHandler<T> {
		TestHandler::<T> {
			res: res,
			handled: 0
		}
	}
}

impl <T: ApiMessage> ConnectionHandler for TestHandler<T> {
	fn request(&mut self, handle: Sender, bytes: Vec<u8>) {
		let res_bytes = encode(self.get_res()).unwrap();
		handle.send(res_bytes);
	}

	fn execute(&mut self, bytes: Vec<u8>) {

	}
}