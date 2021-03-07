mod request;
pub use request::Request;

mod response;
pub use response::Response;

#[derive(Debug)]
enum Method {
  Get,
  Post,
  Put,
  Patch,
  Delete,
  Options,
}

impl Method {
  pub fn from_string(text: &str) -> Self {
    match text {
      "GET" => Self::Get,
      "POST" => Self::Post,
      "PUT" => Self::Put,
      "PATCH" => Self::Patch,
      "DELETE" => Self::Delete,
      "OPTIONS" => Self::Options,
      _ => panic!("Unrecognized HTTP Method: {}", text),
    }
  }
}

#[derive(Debug)]
enum Version {
  One,
  OnePointOne,
  Two,
}

impl Version {
  pub fn from_string(text: &str) -> Self {
    match text {
      "HTTP/1.0" => Self::One,
      "HTTP/1.1" => Self::OnePointOne,
      "HTTP/2.0" => Self::Two,
      _ => panic!("Unimplemented"),
    }
  }
}

#[derive(Debug)]
enum Status {
  Success,
  NoContent,
  NotFound,
}

impl Status {
  pub fn as_string(&self) -> &str {
    match self {
      Self::Success => "HTTP 200 SUCCESS",
      Self::NoContent => "HTTP 204 NO CONTENT",
      Self::NotFound => "HTTP 404 NOT FOUND",
    }
  }
}
