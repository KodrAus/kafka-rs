//TODO: Implement high level client that combines protocol with connection
//Client gets request appropriate for each kind of protocol
//Connection sends request and returns response
//let req = protocol::fetch::Something(param1, param2); <- returns some implementation of Result<ApiRequestMessage<T>>
//let conn = client.get_conn().unwrap(); <- returns some Result<TcpConn>, which the client sets up via ConnectionServer
//let response = conn.execute(req).unwrap(); <- return some Result<ApiResponseMessage<T>>

//Figure out how to keep a separation of concerns between protocol requests and connection
//Perhaps protocol requests should just be serialisable, and have an expected endpoint, conn serialises and ships off

//KafkaConsumerClient -> just listens on a consumer and publishes any messages as they're found for the client
// - Get messages
// - Execute handler logic
// - Advance offset

//Single low-level client that maintains map of cluster data and can execute requests
//Multiple high-level clients that are optimised for a particular scenario