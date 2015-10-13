//TODO: Implement high level client that combines protocol with connection
//Client gets request appropriate for each kind of protocol
//Connection sends request and returns response
//let req = protocol::fetch::Something(param1, param2); <- returns some implementation of Result<ApiRequestMessage<T>>
//let conn = client.get_conn().unwrap(); <- returns some Result<TcpConn>, which the client sets up via ConnectionServer
//let response = conn.execute(req).unwrap(); <- return some Result<ApiResponseMessage<T>>

//Figure out how to keep a separation of concerns between protocol requests and connection
//Perhaps protocol requests should just be serialisable, and have an expected endpoint, conn serialises and ships off