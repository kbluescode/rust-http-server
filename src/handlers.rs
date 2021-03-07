use std::net::TcpStream;
use std::sync::{mpsc::Receiver, Arc, Mutex};

mod listener;
pub use listener::Listener;

mod worker;
pub use worker::Worker;

use crate::http::{Request, Response};

type SharableReceiver<T> = Arc<Mutex<Receiver<T>>>;

// TODO: create http router and handle request data
fn request_handler(_: Request) -> Response {
  let response = Response::default();
  response
}

fn handle_connection(stream: TcpStream, i: u8) {
  let request = Request::from_lines(&stream).unwrap();
  println!("Read from: {}\nRequest Obj: {:?}", i, request);
  let response = request_handler(request);
  println!("Response Obj: {:?}", response);
  response.write(stream);
}
