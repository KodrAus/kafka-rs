#![feature(test)]
extern crate test;
extern crate kafka;

use test::Bencher;

#[bench]
fn it_works(b: &mut Bencher) {
	b.iter(|| {
		
    });
}