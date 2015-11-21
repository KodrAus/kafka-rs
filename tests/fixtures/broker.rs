extern crate mio;
extern crate kafka;

use mio::{ Handler, EventLoop };

pub struct Broker;

//Accepts a connection, reads the data, then sends a response
impl Handler for Broker {
	type Timeout = ();
	type Message = ();
}