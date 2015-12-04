extern crate rustc_serialize;

mod impl_rustc_serialize;

use std::io::Cursor;
use rustc_serialize::{ Encodable, Decodable };
use byteorder::{ ByteOrder, BigEndian, WriteBytesExt };
use self::impl_rustc_serialize::{ encode as enc, decode as dec };
use ::client::protocol::{ ApiMessage, ApiRequestMessage };

pub type EncodingResult<T> = impl_rustc_serialize::EncodingResult<T>;
pub type DecodingResult<T> = impl_rustc_serialize::DecodingResult<T>;

#[derive(RustcEncodable, RustcDecodable)]
pub enum Compression {
	None,
	Gzip,
	Snappy
}

pub fn encode<T: Encodable>(data: &T) -> EncodingResult<Vec<u8>> {
	enc(data)
}

pub fn decode<T: Decodable>(bytes: Vec<u8>) -> DecodingResult<T> {
	dec::<T>(&bytes[..])
}

pub fn usize_as_u32(u: usize) -> u32 {
    u as u32
}

//Preppends the length of a message
pub fn wrap_msg_len(msg: Vec<u8>) -> Vec<u8> {
	let mut msg = msg;
	let mut len_bytes = Vec::with_capacity(4);
	len_bytes.write_u32::<BigEndian>(msg.len() as u32).unwrap();
	len_bytes.append(&mut msg);

	len_bytes
}

//Gets the length of a message if at least the 4 bytes in the u32 are provided
pub fn get_msg_len(msg: Vec<u8>) -> (Option<u32>, Vec<u8>) {
	match msg.len() {
		x if x >=4 => {
			let mut msg = msg;
			let msg_bytes = msg.split_off(4);

			(Some(BigEndian::read_u32(&msg)), msg_bytes)
		},
		_ => (None, msg.clone())
	}
}