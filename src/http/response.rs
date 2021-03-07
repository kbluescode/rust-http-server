use super::Status;
use std::io::Write;

#[derive(Debug)]
pub struct Response {
  status: Status,
  body: Option<String>,
}

impl Response {
  pub fn default() -> Self {
    Response {
      status: Status::Success,
      body: None,
    }
  }

  pub fn write(self, mut writer: impl Write) {
    let message = format!("{}\n", self.status.as_string());
    writer.write(message.as_bytes()).unwrap();
    writer.flush().unwrap();
  }
}
