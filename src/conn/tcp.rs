extern crate mio;
extern crate byteorder;

use std::io::Cursor;
use std::mem;

use mio::{ Handler, EventLoop, Token, EventSet };
use mio::util::Slab;
use mio::tcp::TcpStream;
use super::{ ConnectionMessage };

//The main io loop for clients to interact with
pub struct ConnectionManager {
	connections: Slab<Connection>
}

impl ConnectionManager {
	pub fn new() -> ConnectionManager {
		ConnectionManager {
			connections: Slab::new(1)
		}
	}
}

impl Handler for ConnectionManager {
	type Timeout = ();
	type Message = ConnectionMessage;

	fn notify(&mut self, event_loop: &mut EventLoop<ConnectionManager>, msg: Self::Message) {
		match msg {
			//Sent by clients
			ConnectionMessage::Request(sender, bytes) => {
				//Find an available, compatible connection
				//Store the sender against the connections token id
				//Store the message with the connections token id
				panic!("implement")
			},
			//Sent by clients
			ConnectionMessage::Execute(bytes) => {
				//Find an available, compatible connection
				//Store the message with the connections token id
				panic!("implement")
			},
			//Sent by connections
			ConnectionMessage::Response(bytes) => {
				//Find a saved sender by token id
				//If found, push bytes down channel
				//If not found, ignore and move on
				panic!("implement")
			}
		}
	}

	fn ready(&mut self, event_loop: &mut EventLoop<ConnectionManager>, token: Token, events: EventSet) {
		//Get a connection ready, or deal with closing
		panic!("implement")
	}
}

//A single tcp connection to a broker endpoint
struct Connection {
	socket: TcpStream,
	token: Token,
	state: State
}

enum State {
	Reading(Vec<u8>),
	ReadingMessage(u32, Vec<u8>),
	Writing(Cursor<Vec<u8>>),
	Closed
}