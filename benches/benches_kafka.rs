#![feature(test)]

extern crate test;

#[bench]
#[cfg(feature="test-integration")]
fn single_request_single_broker(b: &mut Bencher) {
	b.iter(|| {
		//Send a single request to a broker in each loop
	});
}

#[bench]
#[cfg(feature="test-integration")]
fn many_requests_single_broker(b: &mut Bencher) {
	b.iter(|| {
		//Send many requests to a single broker in each loop
	});
}

#[bench]
#[cfg(feature="test-integration")]
fn single_command_single_broker(b: &mut Bencher) {
	b.iter(|| {
		//Send a single command (no response) to a single broker in each loop
	});
}

#[bench]
#[cfg(feature="test-integration")]
fn many_commands_single_broker(b: &mut Bencher) {
	b.iter(|| {
		//Send many commands (no response) to a single broker in each loop
	});
}