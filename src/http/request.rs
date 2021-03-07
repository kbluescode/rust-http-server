use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

use super::{Method, Version};

type RequestFormatError = String;

#[derive(Debug)]
pub struct Request {
  method: Method,
  path: String,
  http_version: Version,
  host: String,
  headers: HashMap<String, String>,
  body: Option<String>,
}

impl Request {
  pub fn from_lines(lines: impl Read) -> Result<Self, RequestFormatError> {
    let method;
    let path;
    let http_version;
    let mut host = String::new();
    let mut headers = HashMap::<String, String>::new();

    let reader = BufReader::new(lines);
    let mut line_iter = reader.lines();
    let line = line_iter.next().unwrap();
    match line {
      Err(_) => panic!("couldn't read line"),
      Ok(line) => {
        let mut pieces = line.split(" ");
        method = Method::from_string(pieces.next().unwrap());
        path = pieces.next().unwrap().to_string();
        http_version = Version::from_string(pieces.next().unwrap());
      }
    }

    loop {
      let line = line_iter.next().unwrap();
      if let Ok(line) = line {
        if line.len() == 0 {
          break;
        }
        let mut chunks = line.split(":");
        let key = chunks.next().unwrap();
        match key {
          "Host" => {
            let mut host_name = String::from(chunks.next().unwrap().trim());
            host_name.push(':');
            host_name.push_str(chunks.next().unwrap());
            host = host_name;
          }
          key => {
            let mut val = String::new();
            for chunk in chunks {
              val.push_str(chunk);
            }
            headers.insert(key.to_string(), val.trim().to_string());
          }
        }
      }
    }

    // ex. GET / HTTP/1.1
    // Host: localhost:3000
    // User-Agent: curl/7.58.0
    // Accept: */*

    Ok(Request {
      method,
      path,
      http_version,
      host,
      headers,
      body: None,
    })
  }
}
