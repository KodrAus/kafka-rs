pub mod io_loop;

use mio::Token;
use ::sync::{ Sender };

/// The low-level message that a connection on the event loop will accept
pub enum ConnectionMessage {
	Request(Sender, Vec<u8>),
	Execute(Vec<u8>),
	Response(Vec<u8>)
}

pub trait ConnectionHandler {
	fn request(&mut self, handle: Sender, bytes: Vec<u8>);
	fn execute(&mut self, bytes: Vec<u8>);
}