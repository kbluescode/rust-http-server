use super::SharableReceiver;
use std::net::{TcpListener, TcpStream};
use std::sync::{
  mpsc::{self, Receiver, Sender, TryRecvError},
  Arc,
};
use std::time::Duration;

const LISTENER_TIMEOUT_DUR: Duration = Duration::from_millis(15 * 1000);

pub struct Listener {
  listener: Arc<TcpListener>,
  tcp_sender: Sender<TcpStream>,
  shutdown_receiver: SharableReceiver<bool>,
}

impl Listener {
  pub fn new(
    listener: TcpListener,
    tcp_sender: Sender<TcpStream>,
    shutdown_receiver: SharableReceiver<bool>,
  ) -> Listener {
    Listener {
      listener: Arc::new(listener),
      tcp_sender,
      shutdown_receiver,
    }
  }

  pub fn handler(&self) -> (impl FnOnce() + Send, Receiver<bool>) {
    let listener = Arc::clone(&self.listener);
    let tcp_sender = self.tcp_sender.clone();
    let shutdown_receiver = Arc::clone(&self.shutdown_receiver);

    let (shutdown_sender, worker_shutdown_receiver): (Sender<bool>, Receiver<bool>) =
      mpsc::channel();

    let close = move || {
      let mut running = true;
      while running {
        let shutdown_receiver = shutdown_receiver.lock().unwrap();
        match shutdown_receiver.try_recv() {
          Ok(_) => {
            shutdown_sender.send(true).unwrap();
            running = false;
            continue;
          }
          Err(err) => match err {
            TryRecvError::Empty => {}
            TryRecvError::Disconnected => {
              shutdown_sender.send(true).unwrap();
              running = false;
            }
          },
        }
        match listener.accept() {
          Ok((stream, _)) => {
            stream.set_read_timeout(Some(LISTENER_TIMEOUT_DUR)).unwrap();
            stream
              .set_write_timeout(Some(LISTENER_TIMEOUT_DUR))
              .unwrap();
            tcp_sender.send(stream).unwrap();
          }
          Err(err) => match err.kind() {
            std::io::ErrorKind::WouldBlock => continue,
            _ => running = false,
          },
        }
      }
    };
    (close, worker_shutdown_receiver)
  }
}
