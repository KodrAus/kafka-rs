pub mod tcp_connection;
pub mod tcp_pump;

pub mod tcp {
	pub use super::tcp_pump;
	pub use super::tcp_connection;
}

use std::marker::PhantomData;
use std::sync::mpsc::{ Sender, Receiver };

use ::protocol::messages::ApiMessage;
use ::serialisation::deserialise;

/// The low-level message that a connection on the event loop will accept
pub enum SerialisedConnectionMessage {
	Request(
		Sender<Vec<u8>>,
		Vec<u8>
	),
	Execute(Vec<u8>),
	Response(Vec<u8>)
}

/// Wraps up a receiver in a strongly typed bundle
/// Used by the client to block on responses. Type arg ensures we don't try to receive unexpected messages
/// This type also takes ownership of the receiver, and so it will be disposed of once the handle falls out of scope
pub struct ResponseHandle<T: ApiMessage> {
	handle: Receiver<Vec<u8>>,
	response_type: PhantomData<T>
}

impl <T: ApiMessage> ResponseHandle<T> {
	pub fn new(rx: Receiver<Vec<u8>>) -> ResponseHandle<T> {
		ResponseHandle {
			handle: rx,
			response_type: PhantomData
		}
	}

	pub fn response(&self) -> T {
		//Block on waiting for a response
		let bytes = self.handle.recv().unwrap();

		//Deserialise the result and return
		deserialise::<T>(bytes).unwrap()
	}
}