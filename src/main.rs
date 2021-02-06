use std::net::TcpListener;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread::{self, spawn, JoinHandle};
use std::time::Duration;

const LISTENER_ADDR: &str = "127.0.0.1:3000";
const THREAD_COUNT: u8 = 4;

mod handlers;
use handlers::{Listener, Worker};

fn main() {
  let tcp_listener = TcpListener::bind(LISTENER_ADDR).unwrap();
  tcp_listener
    .set_nonblocking(true)
    .expect("Cannot set non-blocking");

  let mut handlers: Vec<JoinHandle<()>> = vec![];

  let (tcp_sender, tcp_receiver) = mpsc::channel();
  let tcp_receiver = Arc::new(Mutex::new(tcp_receiver));
  let (shutdown_sender, shutdown_receiver) = mpsc::channel();
  let shutdown_receiver = Arc::new(Mutex::new(shutdown_receiver));
  for i in 0..THREAD_COUNT {
    let worker = Worker::new(&tcp_receiver, &shutdown_receiver, i);
    handlers.push(spawn(worker.handler()));
  }

  let listener = Listener::new(tcp_listener, tcp_sender, &shutdown_receiver);
  let listen_handler = spawn(listener.handler());

  thread::sleep(Duration::from_millis(5 * 1000));

  for _ in 0..THREAD_COUNT + 1 {
    shutdown_sender.send(true).unwrap();
  }
  listen_handler.join().unwrap();
  for handler in handlers {
    handler.join().unwrap();
  }
  println!("Exiting!");
}
