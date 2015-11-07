//connection::new()
// - get_conn<Req: ApiMessage, Res: ApiMessage>
//    - request(req: Req) -> response_handle<Res>
//    - response_handle::<Res>::get_response()
//       - Block on waiting for rx from event loop
//       - Deserialise bytes as Res and return

//Getting response will block on wherever it's run
//This design could be heavy for many once-off requests, but should be optimised for the case where the same kind of request is sent very frequently
//So the call to get_conn, which gets a strongly typed client, should always use the same channel for tx/rx to save any unnecessary allocations
//Strongly typed connections should also make use of the broker discovery work done by the root connection::new call. They should be able to contribute to the broker list if it changes