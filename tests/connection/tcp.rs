//TODO: Need a custom handler implementation that parrots tcp responses so we can test ConnectionManager without kafka

#[test]
fn can_create_connection() {
	//Get an address
	//Create a tcp connection and bind to the address
	panic!("implement")
}

#[test]
fn can_send_bytes_through_connection() {
	//Get an address
	//Create a tcp connection and bind
	//Wire up to mio event loop
	//Get mio channel
	//Send request message
	//Assert that the message is received by some custom response handler
	panic!("implement")
}

#[test]
fn can_send_requests_to_connection_manager() {
	panic!("implement")
}

#[test]
fn can_send_commands_to_connection_manager() {
	panic!("implement")
}