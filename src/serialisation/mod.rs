extern crate rustc_serialize;

use rustc_serialize::{ Encodable, Decodable };

mod rustc_serialize_impl;

use rustc_serialize_impl::*;

pub type EncodingResult<T> = bincode::rustc_serialize::EncodingResult<T>;
pub type DecodingResult<T> = bincode::rustc_serialize::DecodingResult<T>;

#[derive(RustcEncodable, RustcDecodable)]
pub enum Compression {
	None,
	Gzip,
	Snappy
}

pub fn serialise<T: Encodable>(data: &T) -> EncodingResult<Vec<u8>> {
	encode(data, SizeLimit::Infinite)
}

pub fn deserialise<T: Decodable>(bytes: Vec<u8>) -> DecodingResult<T> {
	decode::<T>(&bytes[..])
}