extern crate rustc_serialize;

use rustc_serialize::{ Encodable, Decodable };

mod impl_rustc_serialize;
use self::impl_rustc_serialize::*;

pub type EncodingResult<T> = impl_rustc_serialize::EncodingResult<T>;
pub type DecodingResult<T> = impl_rustc_serialize::DecodingResult<T>;

#[derive(RustcEncodable, RustcDecodable)]
pub enum Compression {
	None,
	Gzip,
	Snappy
}

pub fn serialise<T: Encodable>(data: &T) -> EncodingResult<Vec<u8>> {
	encode(data)
}

pub fn deserialise<T: Decodable>(bytes: Vec<u8>) -> DecodingResult<T> {
	decode::<T>(&bytes[..])
}