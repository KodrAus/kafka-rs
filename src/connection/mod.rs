pub mod tcp_connection;
pub mod tcp_pump;

pub mod tcp {
	pub use super::tcp_pump;
	pub use super::tcp_connection;
}

use std::sync::Arc;
use std::sync::mpsc::{ Sender };

/// The low-level message that a connection on the event loop will accept
pub enum SerialisedConnectionMessage {
	Request(
		Sender<Arc<Vec<u8>>>,
		Arc<Vec<u8>>
	),
	Execute(Vec<u8>),
	Response(Arc<Vec<u8>>)
}