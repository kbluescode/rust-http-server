use std::net::{TcpListener, TcpStream};
use std::sync::{
  mpsc::{self, Receiver, Sender},
  Arc, Mutex,
};
use std::thread::{self, spawn, JoinHandle};
use std::time::Duration;

const LISTENER_ADDR: &str = "127.0.0.1:3000";
const THREAD_COUNT: u8 = 4;

mod handlers;
use handlers::{Listener, Worker};

mod http;

fn create_listener(tcp_listener: TcpListener) -> (Listener, Sender<bool>, Receiver<TcpStream>) {
  let (tcp_sender, tcp_receiver) = mpsc::channel();
  let (shutdown_sender, shutdown_receiver) = mpsc::channel();
  let listener = Listener::new(
    tcp_listener,
    tcp_sender,
    Arc::new(Mutex::new(shutdown_receiver)),
  );
  (listener, shutdown_sender, tcp_receiver)
}

fn main() {
  let tcp_listener = TcpListener::bind(LISTENER_ADDR).unwrap();
  tcp_listener
    .set_nonblocking(true)
    .expect("Cannot set non-blocking");

  let mut handlers: Vec<JoinHandle<()>> = vec![];

  let (listener, shutdown_sender, tcp_receiver) = create_listener(tcp_listener);
  let (listen_handler, worker_shutdown_receiver) = listener.handler();
  let listen_handler = spawn(listen_handler);
  let tcp_receiver = Arc::new(Mutex::new(tcp_receiver));
  let worker_shutdown_receiver = Arc::new(Mutex::new(worker_shutdown_receiver));

  for i in 0..THREAD_COUNT {
    let worker = Worker::new(&tcp_receiver, &worker_shutdown_receiver, i);
    handlers.push(spawn(worker.handler()));
  }

  thread::sleep(Duration::from_millis(10 * 1000));

  // TODO: Trap SIGTERM to send shutdown
  shutdown_sender.send(true).unwrap();
  listen_handler.join().unwrap();
  for handler in handlers {
    handler.join().unwrap();
  }
  shutdown_sender.send(true).unwrap();
  println!("Exiting!");
}
