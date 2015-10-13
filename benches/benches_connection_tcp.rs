#![feature(test)]
extern crate test;
extern crate mio;
extern crate kafka;

use test::Bencher;

use std::net;

use kafka::connection::tcp_raw::TcpConn;

#[bench]
fn create_raw_tcp_connection(b: &mut Bencher) {
	b.iter(|| {
		let l = net::TcpListener::bind("127.0.0.1:0").unwrap();
		let addr = l.local_addr().unwrap();

		let conn = TcpConn::new(&addr);
    });
}

#[bench]
fn write_to_raw_tcp_connection(b: &mut Bencher) {
	b.iter(||{

	});
}

#[bench]
fn read_from_raw_tcp_connection(b: &mut Bencher) {
	b.iter(|| {

	});
}