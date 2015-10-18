extern crate mio;
extern crate kafka;

use std::net::SocketAddr;

use mio::tcp::*;

use kafka::connection::tcp::*;

#[test]
fn connection_is_in_read_state_by_default() {
	let addr = "0.0.0.0:0".parse().unwrap();
	let stream = TcpStream::connect(&addr).unwrap();

	//Create a new connection
	let conn = Connection::new(stream, mio::Token(0));

	let isRead = match conn.get_state() {
		&ConnectionState::Reading(..) => true,
		_ => false
	};

	assert!(isRead);
}

#[test]
fn connection_can_transition_to_write_state() {
	panic!("Implement");
}

#[test]
fn connection_can_transition_to_read_state() {
	panic!("Implement");
}

#[test]
fn connection_can_write_to_socket() {
	panic!("Implement");
}

#[test]
fn connection_can_read_from_socket() {
	panic!("Implement");
}