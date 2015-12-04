#![feature(slice_patterns)]
#![feature(custom_derive, plugin)]

extern crate rustc_serialize;
extern crate bincode;
extern crate byteorder;
extern crate mio;

pub mod client;
pub mod encoding;
pub mod conn;

pub use client::protocol::ApiMessage;