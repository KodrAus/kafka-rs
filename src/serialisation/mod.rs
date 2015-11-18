extern crate bincode;
extern crate rustc_serialize;

use bincode::SizeLimit;
use bincode::rustc_serialize::{ encode, decode };

use rustc_serialize::{ Encodable, Decodable };

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