#![feature(slice_patterns)]
#![feature(custom_derive, plugin)]

extern crate rustc_serialize;
extern crate mio;

pub mod client;
pub mod connection;
pub mod protocol;
pub mod serialisation;