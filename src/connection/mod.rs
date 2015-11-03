pub mod tcp_connection;
pub mod tcp_pump;

pub mod tcp {
	pub use super::tcp_pump;
	pub use super::tcp_connection;
}

use std::sync::Arc;
use std::sync::mpsc::{ Sender };

use ::protocol::{ ApiMessage, ApiRequestMessage, ApiResponseMessage };

/// The low-level message that a connection will receive and process
pub enum ConnectionMessage<Req: ApiMessage, Res: ApiMessage> {
	/// An asynchronous Kafka request
	Request(
		Sender<Arc<ApiResponseMessage<Res>>>,
		ApiRequestMessage<Req>
	),
	/// An asynchronous Kafka response
	Response(Arc<ApiResponseMessage<Res>>)
}