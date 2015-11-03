use std::io::Error;

pub trait ToBytes {
	fn to_bytes(&self) -> &[u8];
}

pub trait FromBytes {
	fn from_bytes(buf: &[u8]) -> Result<Self, Error> where Self: Sized;
}

pub enum Compression {
	None,
	Gzip,
	Snappy
}