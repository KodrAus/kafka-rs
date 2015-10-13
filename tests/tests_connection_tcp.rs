//TODO: Block out tcp implementation using mio event loop
//Test locally and as integration tests

extern crate mio;
extern crate kafka;

use std::net;

use mio::*;
use mio::tcp::TcpStream;

use kafka::connection::tcp_raw::TcpConn;

fn get_new_conn() -> TcpConn {
	let l = net::TcpListener::bind("127.0.0.1:0").unwrap();
	let addr = l.local_addr().unwrap();

	TcpConn::new(&addr)
}
#[test]
fn can_create_raw_connection() {
	let conn = get_new_conn();
	println!("{}", conn.address);
}

#[test]
fn can_write_to_raw_connection() {
	let conn = get_new_conn();

	let listener = conn.get_listener();

	//Create an event loop and subscribe a listener to its events on Token(0)
	let event_loop = EventLoop::new().unwrap();
	event_loop.register(&listener, Token::new(0), EventSet::readable(), PollOpt::edge()).unwrap();

	//Write to the conn
	let buf = [0; 1024];
	conn.write(buf).unwrap();

	//Assert the data is written
}

#[test]
fn can_read_from_raw_connection() {
	let conn = get_new_conn();

	let event_loop = EventLoop::new().unwrap();

	//Create a stream and write data to its
	//Assert the conn picks it up
	//TODO: Verify this approach is appropriate
	panic!("Implement");
}

#[test]
fn can_disconnect_raw_connection() {
	panic!("Implement");
}