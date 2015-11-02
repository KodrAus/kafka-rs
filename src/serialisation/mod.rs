use std::io::Error;

pub trait ToBytes {
	fn to_bytes(&self) -> &[u8];
}

pub trait FromBytes {
	type Deserialised;
	fn from_bytes(buf: &[u8]) -> Result<Self::Deserialised, Error>;
}