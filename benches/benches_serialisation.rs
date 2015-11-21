#![feature(test)]

extern crate kafka;
extern crate test;
extern crate bincode;
extern crate rustc_serialize;

use rustc_serialize::{ Encodable, Decodable };

use bincode::SizeLimit;
use bincode::rustc_serialize::{ encode, decode, DecodingError };

use kafka::protocol::*;
use kafka::serialisation::{ serialise, deserialise };

use test::Bencher;

#[derive(RustcEncodable, RustcDecodable)]
pub struct MyRequest {
	pub id: i32,
	pub content: String
}

impl ApiMessage for MyRequest {
	fn get_key(&self) -> i32 {
		self.id
	}
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct MyResponse {
	pub id: i32,
	pub content: String
}

impl ApiMessage for MyResponse {
	fn get_key(&self) -> i32 {
		self.id
	}
}

#[bench]
fn serialise_deserialise_api_request(b: &mut Bencher) {
	let req = ApiRequestMessage { 
		request: MyRequest { 
			id: 1, 
			content: "my request content".to_string() 
		}, 
		api_key: 1,
		api_version: 1,
		correlation_id: 1,
		client_id: "some id".to_string()
	};

	b.iter(||{
		let _ = deserialise::<ApiRequestMessage<MyRequest>>(
			serialise(&req).unwrap()
		).unwrap();
	});
}

#[bench]
fn serialise_deserialise_api_response(b: &mut Bencher) {
	let res = ApiResponseMessage {
		response: MyResponse {
			id: 1,
			content: "my response content".to_string()
		},
		correlation_id: 1
	};

	b.iter(||{
		let _ = deserialise::<ApiResponseMessage<MyResponse>>(
			serialise(&res).unwrap()
		).unwrap();
	});
}