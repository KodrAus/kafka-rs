extern crate rustc_serialize;

#[derive(RustcEncodable, RustcDecodable)]
pub enum Compression {
	None,
	Gzip,
	Snappy
}

//TODO: Encapsulate serialisation once the API is proven