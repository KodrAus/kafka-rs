//Connection takes request bytes as buf, returns response bytes as buf
//This is where mio support will be
//TODO: Need to figure out what the shape of this will be, how concurrency will work etc

//Embrace mio event loop concept; events on reading at minimum for users to subscribe to

extern crate mio;

use std::net::SocketAddr;
use self::mio::tcp::*;

pub struct KafkaConnection {
	address: String,
	stream: TcpStream
}