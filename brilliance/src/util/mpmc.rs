use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::sync::{Arc, Mutex};

/// Constructs a multi-producer, multi-consumer channel.
/// Simlar to stdd:sync::mpsc, but allowing for multiple consuming threads.
pub fn mpmc_channel<T>() -> (Sender<T>, MReceiver<T>) {
	let (sender, receiver) = channel();
	(sender, MReceiver(Arc::new(Mutex::new(receiver))))
}

/// Muti-consumer receiver.
/// Like mpsc::Receiver, but allowing for multiple consuming threads.
pub struct MReceiver<T>(Arc<Mutex<Receiver<T>>>);

impl<T> MReceiver<T> {
	/// Like mpsc::Receiver::recv, but allowing for multiple consuming threads.
	pub fn recv(&mut self) -> Result<T, RecvError> {
		self.0.lock().unwrap().recv()
	}

	pub fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}
/// Like mpsc::Receiver::Iterator, but allowing for multiple consuming threads.
impl<T> Iterator for MReceiver<T> {
	type Item = T;
	fn next(&mut self) -> Option<T> {
		self.0.lock().unwrap().recv().ok()
	}
}
