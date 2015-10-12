//TODO: Block out tcp implementation using mio event loop
//Test locally and as integration tests

extern crate mio;
extern crate kafka;

use std::net;
use std::thread;
use std::io::{ Read, Write };

use mio::{ EventLoop, Handler, Token, EventSet, PollOpt, TryRead, TryWrite };
use mio::tcp::{ TcpListener, TcpStream };

#[test]
fn can_write_to_stream() {
	//Writer handler for the TCP stream
	const N: usize = 16 * 1024 * 1024;
	struct H { amt: usize, socket: TcpStream }

	impl Handler for H {
		type Timeout = ();
		type Message = ();

		//Executed when the event loop is ready
		fn ready(&mut self, event_loop: &mut EventLoop<Self>, token: Token, _events: EventSet) {
			assert_eq!(token, Token(1));

			let b = [0; 1024];
			loop {
				if let Some(amt) = self.socket.try_write(&b).unwrap() {
					self.amt += amt;
				}
				else {
					break
				}
				if self.amt >= N {
					event_loop.shutdown();
					break
				}
			}
		}
	}

	//Bind a TcpListener
	let l = net::TcpListener::bind("127.0.0.1:0").unwrap();
	let addr = l.local_addr().unwrap();

	//Spawn a worker thread that grabs the TcpStream from l and reads its contents to a buffer until finished
	let t = thread::spawn(move || {
		let mut s = l.accept().unwrap().0;
		let mut b = [0; 1024];
		let mut amt = 0;

		while amt < N {
			amt += s.read(&mut b).unwrap();
		}
	});

	//Create a new event loop and register an event handler that writes to the socket being read on another thread
	let mut e = EventLoop::new().unwrap();
	let s = TcpStream::connect(&addr).unwrap();

	e.register(&s, Token(1)).unwrap();

	let mut h = H { amt: 0, socket: s };
	e.run(&mut h).unwrap();
	
	t.join().unwrap();
}