use std::{collections::HashMap, str::FromStr};
use anyhow::Result;
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

impl FromStr for Method {
    type Err = ();

    fn from_str(method: &str) -> Result<Self, Self::Err> {
        match method {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            "DELETE" => Ok(Method::Delete),
            _ => Err(()),
        }
    }
}

pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
}

impl Request {
    pub fn parse(raw_request: Vec<u8>) -> Result<Self> {
        let request_str = String::from_utf8(raw_request)?;

        let (request_line, headers) = request_str.split_once("\r\n").unwrap();

        let (method, path, _) = Self::parse_request_line(request_line)?;
        let headers = Self::parse_headers(headers)?;

        Ok(Request {
            method,
            path,
            headers,
        })
        
    }

    fn parse_request_line(request_line: &str) -> Result<(Method, String, String)> {
        let mut parts = request_line.split_whitespace();

        let method = parts.next().expect("Missing method");
        let path = parts.next().expect("Missing path");
        let version = parts.next().expect("Missing version");

        let method = Method::from_str(method).expect("Invalid method");
        
        Ok((method, path.to_string(), version.to_string()))
    }

    fn parse_headers(headers: &str) -> Result<HashMap<String, String>> {
        let mut header_map = HashMap::new();

        for line in headers.lines() {
            if let Some((key, value)) = line.split_once(": ") {
                header_map.insert(key.to_string(), value.to_string());
            }
        }

        Ok(header_map)
    }
}
