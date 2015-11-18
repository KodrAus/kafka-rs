extern crate mio;

use mio::{ Handler };

use super::{ SerialisedConnectionMessage };

pub struct TcpConnection;

impl Handler for TcpConnection {
	type Timeout = ();
	type Message = SerialisedConnectionMessage;
}