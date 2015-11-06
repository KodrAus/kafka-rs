# kafka-rs

Very early attempt at building an [Apache Kafka](http://kafka.apache.org/) client in [Rust](https://www.rust-lang.org/)

## Goals

To provide an asynchronous, high-level, strongly-typed, well tested and intuitive Kafka driver in Rust. The aim is to support two main use-cases with Kafka:

- Once-off message sending / receiving through a standard connection with request/response methods
- Streaming scenarios where messages are 'pumped' from a topic and handled by a filter, which in turn pumps its own messages to another topic. So each filter handles messages of a single type as they arrive, and may choose to publish messages of a single type in response.

Both use-cases will build off a standard Kafka io base that will be suitable for any communication with a kafka cluster.
