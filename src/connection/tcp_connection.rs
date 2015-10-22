//Two-way TCP stream

extern crate mio;
extern crate bytes;

use self::mio::{ TryRead, TryWrite };
use self::mio::tcp::*;
use self::mio::util::Slab;
use self::bytes::{ Buf, Take };
use std::mem;
use std::io::Cursor;

const MAX_LINE: usize = 128;

//The mio handler for happenings on the tcp stream
struct Server {
	listener: TcpListener,
	connections: Slab<Connection>
}

impl Server {
	fn new (listener: TcpListener, start_token: mio::Token, range: i32) -> Server {
		let slab = Slab::new_starting_at(start_token, range);

		Server {
			listener: listener,
			connections: slab
		}
	}
}

impl mio::Handler for Server {
	type timeout = ();
	type Message = ();

	//When ready, create a connection and write the buffered data to it
}

//The mio handler for working with a tcp stream
#[derive(Debug)]
pub struct Connection {
	socket: TcpStream,
	token: mio::Token,
	state: ConnectionState
}

impl Connection {
	pub fn new(socket: TcpStream, token: mio::Token) -> Connection {
		Connection {
			socket: socket, 
			token: token,
			state: ConnectionState::Reading(Vec::with_capacity(MAX_LINE))
		}
	}

	pub fn get_state(&self) -> &ConnectionState {
		&self.state
	}

	pub fn ready(&mut self, event_loop: &mut mio::EventLoop<Handler>, events: mio::EventSet) {
		match self.state {
			ConnectionState::Reading(..) => {
				assert!(events.is_readable(), "unexpected events");
				self.read(event_loop)
			}
			ConnectionState::Writing(..) => {
				assert!(events.is_writable(), "unexpected events");
			}
			_ => unimplemented!();
		}
	}

	pub fn read(&mut self, event_loop: &mut mio::EventLoop<Handler>) {
		match self.socket.try_read_buf(self.state.mut_read_buf()) {
			Ok(Some(0)) => {
				self.state = ConnectionState::Closed;
			}
			Ok(Some(n)) => {
				println!("read {} bytes", n);

				self.state.try_transition_to_writing();
				self.reregister(event_loop);
			}
			Ok(None) => {
				self.reregister(event_loop);
			}
			Err(e) => {
				panic!("got an error trying to read: err={:?}", e);
			}
		}
	}

	pub fn write(&mut self, event_loop: &mut mio::EventLoop<Handler>) {
		match self.socket.try_write_uf(self.state.mut_write_buf()) {
			Ok(Some(_)) => {
				self.state.try_transition_to_reading();
				self.reregister(event_loop);
			}
			Ok(None) => {
				self.reregister(event_loop);
			}
			Err(e) => {
				panic!("got an error trying to write: err={:?}", e);
			}
		}
	}

	fn reregister(&self, event_loop: mio::EventLoop<Handler>) {
		event_loop.reregister(&self.socket, self.token, self.state.event_set(), mio::PollOpt::oneshot()).unwrap();
	}

	fn is_closed(&self) -> bool {
		match self.state {
			ConnectionState::Closed => true,
			_ => false
		}
	}
}

#[derive(Debug)]
pub enum ConnectionState {
	Reading(Vec<u8>),
	Writing(Take<Cursor<Vec<u8>>>),
	Closed
}

impl ConnectionState {
	fn event_set(&self) -> mio::EventSet {
		match *self {
			ConnectionState::Reading(..) => mio::EventSet::readable(),
			ConnectionState::Writing(..) => mio::EventSet::writable(),
			_ => mio::EventSet::none()
		}
	}

	//TODO: Tidy these up, reduce repeated code
	//Make the state self inclusive, rather than throwing if used in wrong context
	fn mut_read_buf(&mut self) -> &mut Vec<u8> {
		match *self {
			ConnectionState::Reading(ref mut buf) => buf,
			_ => panic!("connection is not in reading state")
		}
	}

	fn read_buf(&self) -> &[u8] {
		match *self {
			ConnectionState::Reading(ref buf) => buf,
			_ => panic!("connection is not in reading state")
		}
	}

	fn try_transition_to_reading(&mut self) {
		if !self.write_buf().has_remaining() {
			let cursor = mem::replace(self, ConnectionState::Closed)
				.unwrap_write_buf()
				.into_inner();

			let pos = cursor.position() as usize;
			let mut buf = cursor.into_inner();
			
			//Drop all data that has been written to the client
			let mut buf = buf.drain(0..pos).collect();

			*self = ConnectionState::Reading(buf);

			//Check for any new lines that have already been read
			self.try_transition_to_writing();
		}
	}

	fn unwrap_read_buf(self) -> Vec<u8> {
		match self {
			ConnectionState::Reading(buf) => buf,
			_ => panic!("connection is not in reading state")
		}
	}

	fn mut_write_buf(&mut self) -> &mut Take<Cursor<Vec<u8>>> {
		match *self {
			ConnectionState::Writing(ref mut buf) => buf,
			_ => panic!("connection is not in writing state")
		}
	}

	fn write_buf(&self) -> &Take<Cursor<Vec<u8>>> {
		match *self {
			ConnectionState::Writing(ref buf) => buf,
			_ => panic!("connection is not in writing state")
		}
	}

	fn try_transition_to_writing(&mut self) {
		if let Some(pos) = self.read_buf().iter().position(|b| *b == b'\n') {
			let buf = mem::replace(self, ConnectionState::Closed)
				.unwrap_read_buf();

			let buf = Cursor::new(buf);

			*self = ConnectionState::Writing(Take::new(buf, pos + 1));
		}
	}

	fn unwrap_write_buf(self) -> Take<Cursor<Vec<u8>>> {
		match self {
			ConnectionState::Writing(buf) => buf,
			_ => panic!("connection is not in writing state")
		}
	}
}