pub mod tcp_connection;
pub mod tcp_pump;

pub mod tcp {
	pub use super::tcp_pump;
	pub use super::tcp_connection;
}

use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::mpsc::{ Sender, Receiver };

use ::protocol::messages::ApiMessage;

/// The low-level message that a connection on the event loop will accept
enum SerialisedConnectionMessage {
	Request(
		Sender<Arc<Vec<u8>>>,
		Arc<Vec<u8>>
	),
	Execute(Arc<Vec<u8>>),
	Response(Arc<Vec<u8>>)
}

/// Wraps up a receiver in a strongly typed bundle
/// Used by the client to block on responses. Type arg ensures we don't try to receive unexpected messages
/// This type also takes ownership of the receiver, and so it will be disposed of once the handle falls out of scope
pub struct ResponseHandle<T: ApiMessage> {
	handle: Receiver<Arc<Vec<u8>>>,
	response_type: PhantomData<T>
}

impl <T: ApiMessage> ResponseHandle<T> {
	pub fn new(rx: Receiver<Arc<Vec<u8>>>) -> ResponseHandle<T> {
		ResponseHandle {
			handle: rx,
			response_type: PhantomData
		}
	}

	pub fn get_response(&self) -> T {
		//let res = self.handle.recv();
		//deserialise res as T
		panic!("implement")
	}
}