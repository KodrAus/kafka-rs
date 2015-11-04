#![feature(test)]

extern crate test;
extern crate kafka;

use test::Bencher;

#[bench]
fn serialise_deserialise_api_request(b: &mut Bencher) {
	b.iter(||{

	});
}

#[bench]
fn serialise_deserialise_api_response(b: &mut Bencher) {
	b.iter(||{

	});
}