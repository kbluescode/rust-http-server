const BUFFER_SIZE: usize = 1024;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};

pub type SharableReceiver<T> = Arc<Mutex<Receiver<T>>>;

fn handle_connection(mut stream: TcpStream, i: u8) {
  let mut buffer = [0; BUFFER_SIZE];
  stream.read(&mut buffer).unwrap();
  stream.write(b"derp").unwrap();
  stream.flush().unwrap();
  println!(
    "Read from {}\nRequest: {}",
    i,
    String::from_utf8_lossy(&buffer[..])
  );
}

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
    Box::new(move || {
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
    })
  }
}

pub struct Listener<'a> {
  listener: Arc<TcpListener>,
  tcp_sender: Sender<TcpStream>,
  shutdown_receiver: &'a SharableReceiver<bool>,
}

impl<'a> Listener<'a> {
  pub fn new(
    listener: TcpListener,
    tcp_sender: Sender<TcpStream>,
    shutdown_receiver: &'a SharableReceiver<bool>,
  ) -> Listener {
    Listener {
      listener: Arc::new(listener),
      tcp_sender,
      shutdown_receiver,
    }
  }

  pub fn handler(&self) -> impl FnOnce() + Send {
    let listener = Arc::clone(&self.listener);
    let tcp_sender = self.tcp_sender.clone();
    let shutdown_receiver = Arc::clone(&self.shutdown_receiver);
    Box::new(move || {
      println!("Hello, listener!");
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
        match listener.accept() {
          Ok((stream, _)) => tcp_sender.send(stream).unwrap(),
          Err(err) => match err.kind() {
            std::io::ErrorKind::WouldBlock => continue,
            _ => running = false,
          },
        }
      }
      println!("Goodbye, listener!");
    })
  }
}
