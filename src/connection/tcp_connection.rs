use std::sync::Arc;
use std::sync::mpsc::{ Sender };

use ::protocol::{ Message, ApiRequestMessage, ApiResponseMessage };

//TODO: Find out whether Arc is the best type to be using here. Seems ok to me
pub enum ConnectionMessage<Req: Message, Res: Message> {
	Request(
		//Where to send the response to
		Sender<Arc<ApiResponseMessage<Res>>>,
		//The request message
		ApiRequestMessage<Req>
	),
	Response(Arc<ApiResponseMessage<Res>>)
}