use std::sync::mpsc::Receiver;
use super::protocol::ApiMessage;
use ::encoding::decode;

enum ResponseHandleState<T: ApiMessage> {
	AwaitingResponse(Receiver<Vec<u8>>),
	HasResponse(T)
}

enum ResponseMultiState {
	SingleResponse,
	SomeResponses(usize),
	InfiniteResponses
}

impl ResponseMultiState {
	pub fn set_remaining_responses(&mut self, left: usize) {
		*self = ResponseMultiState::SomeResponses(left);
	}
}

impl <T: ApiMessage> ResponseHandleState<T> {
	fn block_for_response(&self) -> Option<T> {
		match *self {
			ResponseHandleState::AwaitingResponse(ref handle) => {
				let bytes = handle.recv().unwrap();
				let msg = decode::<T>(bytes).unwrap();

				Some(msg)
			},
			_ => None
		}
	}

	fn get_cached_response(&self) -> Option<T> {
		match *self {
			ResponseHandleState::HasResponse(ref msg) => Some(msg.clone()),
			_ => None
		}
	}

	fn set_cached_response(&mut self, msg: T) {
		*self = ResponseHandleState::HasResponse(msg);
	}

	pub fn get_response(&mut self, multi_state: &mut ResponseMultiState) -> Option<T> {
		match *multi_state {
			//1 response
			ResponseMultiState::SingleResponse => {
				if let Some(msg) = self.block_for_response() {
					self.set_cached_response(msg);
				}
				self.get_cached_response()
			},
			//n responses
			ResponseMultiState::SomeResponses(n) if n > 0 => {
				multi_state.set_remaining_responses(n - 1);
				self.block_for_response()
			},
			//0 responses
			ResponseMultiState::SomeResponses(_) => {
				None
			},
			//infinite responses
			ResponseMultiState::InfiniteResponses => {
				self.block_for_response()
			}
		}
	}
}

/// Wraps up a receiver in a strongly typed bundle
/// Used by the client to block on responses. Type arg ensures we don't try to receive unexpected messages
/// This type also takes ownership of the receiver, and so it will be disposed of once the handle falls out of scope
pub struct ResponseHandle<T: ApiMessage> {
	state: ResponseHandleState<T>,
	multi: ResponseMultiState
}

impl <T: ApiMessage> ResponseHandle<T> {
	pub fn new(rx: Receiver<Vec<u8>>) -> ResponseHandle<T> {
		ResponseHandle {
			state: ResponseHandleState::AwaitingResponse(rx),
			multi: ResponseMultiState::SingleResponse
		}
	}
	
	pub fn from_response(msg: T) -> ResponseHandle<T> {
		ResponseHandle {
			state: ResponseHandleState::HasResponse(msg),
			multi: ResponseMultiState::SingleResponse
		}
	}
	
	pub fn response(&mut self) -> Option<T> {
		self.state.get_response(&mut self.multi)
	}
}