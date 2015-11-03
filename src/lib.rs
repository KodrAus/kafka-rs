#![feature(custom_derive, plugin)]
#![allow(unstable)]
extern crate bincode;
extern crate serde;
extern crate mio;

pub mod connection;
pub mod protocol;
pub mod serialisation;