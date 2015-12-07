extern crate bincode;
extern crate kafka;

use kafka::client::protocol::*;
use kafka::encoding::{ encode, decode, wrap_msg_len, get_msg_len };
use ::fixtures::*;

#[test]
fn can_encode_and_decode_api_requests() {
	//Create a request object
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

	//Serialise the request and then deserialise
	let bytes = encode(&req).unwrap();
	let des_req = decode::<ApiRequestMessage<MyRequest>>(bytes).unwrap();

	assert!(req.request.content == des_req.request.content);
}

#[test]
fn can_wrap_and_get_encoded_message_length() {
	//Create a request object
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

	//Serialise the request and then deserialise
	let bytes = encode(&req).unwrap();
	let len = bytes.len();
	let bytes_with_len = wrap_msg_len(bytes);

	//Make sure the length is appended
	assert!(bytes_with_len.len() == len + 4);

	let (_len, _bytes) = get_msg_len(bytes_with_len);
	assert!(_len.unwrap() == len as u32);

	//Make sure the message is correct
	let des_req = decode::<ApiRequestMessage<MyRequest>>(_bytes).unwrap();
	assert!(req.request.content == des_req.request.content);
}

#[test]
fn get_msg_len_is_none_if_less_than_4_bytes_provided() {
	//Create a buffer with less than 4 bytes. We can't decode the message length yet
	let bytes_with_len = vec![ 1, 2, 3 ];

	let (len, bytes) = get_msg_len(bytes_with_len);

	//The length is none, and the returned buffer is the same as we passed in
	assert!(len.is_none());
	assert!(bytes.len() == 3);
}