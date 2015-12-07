extern crate test;
extern crate mio;
extern crate kafka;

use test::Bencher;
use kafka::client::ResponseHandle;
use kafka::client::protocol::ApiResponseMessage;
use ::fixtures::*;

#[bench]
fn get_cached_response(b: &mut Bencher) {
	let mut handle = ResponseHandle::from_response(ApiResponseMessage {
		response: MyResponse {
			id: 1,
			content: "my response content".to_string()
		},
		correlation_id: 1
	});

	b.iter(move || {
		let _ = handle.response();
	});
}