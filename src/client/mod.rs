extern crate mio;

pub mod response_handle;

pub use self::response_handle::*;

use std::sync::mpsc;
use std::any::Any;

use mio::Sender;

use ::connection::BufferedConnectionMessage;
use ::protocol::{ RequestResponseMessage, ApiMessage, ApiRequestMessage, ApiResponseMessage };
use ::serialisation::{ serialise, deserialise };

pub struct Client {
	app: String,
	io: Sender<BufferedConnectionMessage>
}

impl Client {
	pub fn new(app: String, io: Sender<BufferedConnectionMessage>) -> Client {
		Client {
			app: app,
			io: io
		}
	}

	pub fn request<Req: ApiMessage, Res: ApiMessage>(&self, msg: ApiRequestMessage<Req>) -> Result<ResponseHandle<ApiResponseMessage<Res>>, String> {
		let (tx, rx) = mpsc::channel();

		//Serialise the request message
		let bytes = serialise(&RequestResponseMessage::new(&msg)).unwrap();
		
		//Send to event loop
		self.io.send(BufferedConnectionMessage::Request(tx, bytes));

		//Send the request as a notify to mio
		Ok(ResponseHandle::new(rx))
	}
}