use ::protocol::ApiMessage;
use ::protocol::encoding::decode;
use ::sync::{ Sender, Receiver, channel };

/// Wraps up a receiver in a strongly typed bundle
/// Used by the client to block on responses. Type arg ensures we don't try to receive unexpected messages
/// This type also takes ownership of the receiver, and so it will be disposed of once the handle falls out of scope
pub struct ResponseHandle<T: ApiMessage> {
	state: ResponseHandleState<T>,
	stream: ResponseStreamType
}

impl <T: ApiMessage> ResponseHandle<T> {
	pub fn new(rx: Receiver) -> ResponseHandle<T> {
		ResponseHandle {
			state: ResponseHandleState::AwaitingResponse(rx),
			stream: ResponseStreamType::SingleResponse
		}
	}

	pub fn some(rx: Receiver, count: usize) -> ResponseHandle<T> {
		ResponseHandle {
			state: ResponseHandleState::AwaitingResponse(rx),
			stream: ResponseStreamType::SomeResponses(count)
		}
	}

	pub fn streaming(rx: Receiver) -> ResponseHandle<T> {
		ResponseHandle {
			state: ResponseHandleState::AwaitingResponse(rx),
			stream: ResponseStreamType::StreamingResponses
		}
	}
	
	pub fn from_response(msg: T) -> ResponseHandle<T> {
		ResponseHandle {
			state: ResponseHandleState::HasResponse(msg),
			stream: ResponseStreamType::SingleResponse
		}
	}
	
	pub fn response(&mut self) -> Option<T> {
		self.state.get_response(&mut self.stream)
	}
}

enum ResponseHandleState<T: ApiMessage> {
	AwaitingResponse(Receiver),
	HasResponse(T)
}

enum ResponseStreamType {
	SingleResponse,
	SomeResponses(usize),
	StreamingResponses
}

impl ResponseStreamType {
	pub fn set_remaining_responses(&mut self, left: usize) {
		*self = ResponseStreamType::SomeResponses(left);
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

	pub fn get_response(&mut self, stream: &mut ResponseStreamType) -> Option<T> {
		match *stream {
			//1 response
			ResponseStreamType::SingleResponse => {
				if let Some(msg) = self.block_for_response() {
					self.set_cached_response(msg);
				}
				self.get_cached_response()
			},
			//n responses
			ResponseStreamType::SomeResponses(n) if n > 0 => {
				stream.set_remaining_responses(n - 1);
				self.block_for_response()
			},
			//0 responses
			ResponseStreamType::SomeResponses(_) => {
				None
			},
			//infinite responses
			ResponseStreamType::StreamingResponses => {
				self.block_for_response()
			}
		}
	}
}
