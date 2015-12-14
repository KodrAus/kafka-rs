extern crate kafka;
extern crate test;

use kafka::protocol::*;
use kafka::protocol::encoding::{ encode, decode };
use ::fixtures::*;

use test::Bencher;

#[bench]
fn encode_decode_api_request_sml(b: &mut Bencher) {
	let req = ApiRequestMessage { 
		request: MyRequest { 
			id: 1, 
			content: "my request content".to_string() 
		}, 
		api_key: 1,
		api_version: 1,
		correlation_id: 1,
		client_id: "some id".to_string()
	};

	b.iter(||{
		let _ = decode::<ApiRequestMessage<MyRequest>>(
			encode(&req).unwrap()
		).unwrap();
	});
}

#[bench]
fn encode_decode_api_response_sml(b: &mut Bencher) {
	let res = ApiResponseMessage {
		response: MyResponse {
			id: 1,
			content: "my response content".to_string()
		},
		correlation_id: 1
	};

	b.iter(||{
		let _ = decode::<ApiResponseMessage<MyResponse>>(
			encode(&res).unwrap()
		).unwrap();
	});
}