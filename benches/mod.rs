#![feature(test)]

extern crate rustc_serialize;
extern crate stopwatch;
extern crate mio;
extern crate test;
extern crate kafka;

pub mod fixtures;
pub mod encoding;
pub mod client;