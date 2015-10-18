extern crate mio;
extern crate bytes;

use self::mio::{ TryRead, TryWrite };
use self::mio::tcp::*;
use self::mio::util::Slab;
use self::bytes::{ Buf, Take };
use std::mem;
use std::io::Cursor;

const MAX_LINE: usize = 128;

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