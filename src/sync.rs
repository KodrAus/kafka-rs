use std::sync::mpsc;

//Bits for sending/receiving bytes between threads
pub type Sender = mpsc::Sender<Vec<u8>>;
pub type Receiver = mpsc::Receiver<Vec<u8>>;
pub fn channel() -> (Sender, Receiver) {
	mpsc::channel()
}