pub mod tcp_connection;
pub mod tcp_pump;

pub mod tcp {
	pub use super::tcp_pump;
	pub use super::tcp_connection;
}

use std::sync::Arc;
use std::sync::mpsc::{ Sender };

use ::protocol::{ ApiMessage, ApiRequestMessage, ApiResponseMessage };

pub enum ConnectionMessage<Req: ApiMessage, Res: ApiMessage> {
	Request(
		Sender<Arc<ApiResponseMessage<Res>>>,
		ApiRequestMessage<Req>
	),
	Response(Arc<ApiResponseMessage<Res>>)
}