#![feature(test)]

extern crate test;
extern crate mio;
extern crate kafka;

use test::Bencher;
use std::net::SocketAddr;

use mio::tcp::*;

use kafka::connection::tcp::*;

#[bench]
fn create_tcp_connection(b: &mut Bencher) {
	b.iter(||{

	});
}

#[bench]
fn write_to_tcp_connection(b: &mut Bencher) {
	b.iter(||{

	});
}

#[bench]
fn read_from_tcp_connection(b: &mut Bencher) {
	b.iter(|| {

	});
}