pub mod tcp;

use std::sync::mpsc::{ Sender };

/// The low-level message that a connection on the event loop will accept
pub enum BufferedConnectionMessage {
	Request(
		Sender<Vec<u8>>,
		Vec<u8>
	),
	Execute(Vec<u8>),
	Response(Vec<u8>)
}

//Msg -> RequestMessage<Msg> -> RequestResponseMessage -> Vec<u8> -> BufferedConnectionMessage::Request