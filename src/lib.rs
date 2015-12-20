#![feature(slice_patterns, custom_derive, plugin)]

extern crate rustc_serialize;
extern crate bincode;
extern crate byteorder;
extern crate mio;

pub mod client;
pub mod protocol;
pub mod conn;
pub mod sync;

pub use protocol::ApiMessage;