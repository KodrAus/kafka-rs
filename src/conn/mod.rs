pub mod io_loop;

use std::sync::mpsc::{ Sender };

/// The low-level message that a connection on the event loop will accept
pub enum ConnectionMessage {
	Request(
		Sender<Vec<u8>>,
		Vec<u8>
	),
	Execute(Vec<u8>),
	Response(Vec<u8>)
}