#![feature(slice_patterns)]
#![feature(custom_derive, plugin)]

extern crate bincode;
extern crate rustc_serialize;
extern crate mio;

pub mod connection;
pub mod protocol;
pub mod serialisation;