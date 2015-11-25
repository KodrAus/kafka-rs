use std::sync::mpsc::{ Receiver };

use ::protocol::messages::ApiMessage;
use ::serialisation::deserialise;

enum ResponseHandleState<T: ApiMessage> {
	WaitingOnResponse(Receiver<Vec<u8>>),
	HaveResponse(T)
}

impl <T: ApiMessage> ResponseHandleState<T> {
	fn block_for_response(&self) -> Option<T> {
		match *self {
			ResponseHandleState::WaitingOnResponse(ref handle) => {
				let bytes = handle.recv().unwrap();
				let msg = deserialise::<T>(bytes).unwrap();

				Some(msg)
			},
			_ => None
		}
	}

	fn get_cached_response(&self) -> Option<&T> {
		match *self {
			ResponseHandleState::HaveResponse(ref msg) => Some(msg),
			_ => None
		}
	}

	pub fn get_response(&mut self) -> Option<&T> {
		if let Some(msg) = self.block_for_response() {
			self.transition_to_cached_response(msg);
		}

		self.get_cached_response()
	}

	fn transition_to_cached_response(&mut self, msg: T) {
		*self = ResponseHandleState::HaveResponse(msg);
	}
}

/// Wraps up a receiver in a strongly typed bundle
/// Used by the client to block on responses. Type arg ensures we don't try to receive unexpected messages
/// This type also takes ownership of the receiver, and so it will be disposed of once the handle falls out of scope
pub struct ResponseHandle<T: ApiMessage> {
	state: ResponseHandleState<T>
}

impl <T: ApiMessage> ResponseHandle<T> {
	pub fn new(rx: Receiver<Vec<u8>>) -> ResponseHandle<T> {
		ResponseHandle {
			state: ResponseHandleState::WaitingOnResponse(rx)
		}
	}

	pub fn response(&mut self) -> Option<&T> {
		self.state.get_response()
	}
}