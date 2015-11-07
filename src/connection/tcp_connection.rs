extern crate mio;

use mio::{ Token, EventLoop, Handler };

use super::{ SerialisedConnectionMessage };

pub struct TcpConnection {
	token: Token
}

impl Handler for TcpConnection {
	type Timeout = ();
	type Message = SerialisedConnectionMessage;
}