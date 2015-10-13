//Connection takes request bytes as buf, returns response bytes as buf
//This is where mio support will be
//TODO: Need to figure out what the shape of this will be, how concurrency will work etc

//Embrace mio event loop concept; events on reading at minimum for users to subscribe to

extern crate mio;
extern crate bytes;

use std::net::SocketAddr;
use std::io;

use self::mio::*;
use self::mio::tcp::*;

use self::bytes::{ ByteBuf, MutByteBuf, SliceBuf };

use self::mio::util::Slab;

//Handler for connection

//Single TCP connection to Kafka
pub struct TcpConn {
	pub address: String,
	stream: TcpStream,
	buffer: Option<ByteBuf>,
	mut_buffer: Option<MutByteBuf>,
	token: Option<Token>,
	interest: EventSet
}

impl TcpConn {
	//Create a new TcpConn
	pub fn new(addr: &SocketAddr) -> TcpConn {
		let stream = TcpStream::connect(addr).unwrap();

		TcpConn {
			address: addr.to_string(),
			stream: stream,
			buffer: None,
			mut_buffer: Some(ByteBuf::mut_with_capacity(2048)),
			token: None,
			interest: EventSet::hup()
		}
	}
}