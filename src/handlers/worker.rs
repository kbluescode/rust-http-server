use super::{handle_connection, SharableReceiver};
use std::net::TcpStream;
use std::sync::{mpsc::TryRecvError, Arc};

pub struct Worker<'a> {
  tcp_receiver: &'a SharableReceiver<TcpStream>,
  shutdown_receiver: &'a SharableReceiver<bool>,
  num: u8,
}

impl<'a> Worker<'a> {
  pub fn new(
    tcp_receiver: &'a SharableReceiver<TcpStream>,
    shutdown_receiver: &'a SharableReceiver<bool>,
    num: u8,
  ) -> Worker<'a> {
    Worker {
      tcp_receiver,
      shutdown_receiver,
      num,
    }
  }

  pub fn handler(&self) -> impl FnOnce() + Send {
    let tcp_receiver = Arc::clone(self.tcp_receiver);
    let shutdown_receiver = Arc::clone(self.shutdown_receiver);
    let i = self.num;
    move || {
      println!("Hello, {}!", i);
      let mut running = true;
      while running {
        let shutdown_receiver = shutdown_receiver.lock().unwrap();
        match shutdown_receiver.try_recv() {
          Ok(_) => {
            running = false;
            continue;
          }
          Err(err) => match err {
            TryRecvError::Empty => {}
            TryRecvError::Disconnected => running = false,
          },
        }
        let tcp_receiver = tcp_receiver.lock().unwrap();
        match tcp_receiver.try_recv() {
          Ok(stream) => handle_connection(stream, i),
          Err(err) => match err {
            TryRecvError::Empty => {}
            TryRecvError::Disconnected => {
              println!("{} disconnected", i);
              running = false
            }
          },
        }
      }
      println!("Goodbye, {}!", i);
    }
  }
}
