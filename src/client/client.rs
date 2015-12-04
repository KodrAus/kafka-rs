extern crate mio;

use std::sync::mpsc;
use mio::Sender;
use super::protocol::{
	ApiMessage, 
	ApiRequestMessage, 
	ApiResponseMessage 
};
use ::encoding::{ encode, decode };
use super::ResponseHandle;

use ::conn::ConnectionMessage;

pub struct Client {
	app: String,
	io: Sender<ConnectionMessage>
}

impl Client {
	pub fn new(app: String, io: Sender<ConnectionMessage>) -> Client {
		Client {
			app: app,
			io: io
		}
	}

	pub fn request<Req: ApiMessage, Res: ApiMessage>(&self, msg: ApiRequestMessage<Req>) -> Result<ResponseHandle<ApiResponseMessage<Res>>, String> {
		let (tx, rx) = mpsc::channel();

		//Encode the message, then append its length to the front
		let mut msgbytes = encode(&msg).unwrap();
		let mut bytes = encode(&msgbytes.len()).unwrap();
		bytes.append(&mut msgbytes);

		self.io.send(ConnectionMessage::Request(tx, bytes));

		Ok(ResponseHandle::new(rx))
	}
}