use std::sync::Arc;
use std::sync::mpsc::{ Sender };

use ::protocol::{ Message, ApiRequestMessage, ApiResponseMessage };

pub enum ConnectionMessage<Req: Message, Res: Message> {
	Request(
		//Where to send the response to
		Sender<Arc<ApiResponseMessage<Res>>>,
		//The request message
		ApiRequestMessage<Req>
	),
	Response(Arc<ApiResponseMessage<Res>>)
}