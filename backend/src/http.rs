use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{Error, ErrorKind, Result};
use std::net::TcpStream;

pub struct Target {
    pub path: String,
    pub query: Option<HashMap<String, Option<String>>>,
}

pub struct Request {
    stream: TcpStream,
    method: String,
    target: Target,
    version: String,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl Request {
    pub fn new(stream: TcpStream) -> Result<Request> {
        // Create a buffered reader for the stream
        let mut reader = BufReader::new(stream);

        // Read the first line
        let mut first_line = String::new();
        reader.read_line(&mut first_line)?;

        // Split the first line into its 3 parts
        let first_line: Vec<_> = first_line.split_whitespace().collect();

        // Ensure all 3 parts are there
        if first_line.len() != 3 {
            let error = Error::new(ErrorKind::Other, "Invalid First Line");
            return Err(error);
        }

        // Split the target into path and query
        let target: Vec<_> = first_line[1].splitn(2, "?").collect();
        let path = target[0];

        let query: Option<HashMap<_, _>> = if target.len() == 2 {
            Some(parse_query(target[1].to_string()))
        } else {
            None
        };

        // Finalize the target
        let target = Target {
            path: path.to_string(),
            query,
        };

        // Get the headers
        let mut headers = HashMap::new();

        loop {
            let mut line = String::new();
            reader.read_line(&mut line)?;

            // A blank link signals the end of the headers
            if line.trim().len() == 0 {
                break;
            }

            // Split the line by colon
            let line: Vec<_> = line.splitn(2, ":").map(|p| p.trim()).collect();

            // Ensure there are 2 parts
            if line.len() != 2 {
                let error = Error::new(ErrorKind::Other, "Invalid Header");
                return Err(error);
            }

            // Add to headers hashmap
            headers.insert(line[0].to_string(), line[1].to_string());
        }

        let mut body = None;

        // Get the body
        if let Some(len) = headers.get("Content-Length") {
            // Parse `len` into a number
            let len = match len.parse::<u32>() {
                Ok(len) => len,
                Err(_) => {
                    let error = Error::new(ErrorKind::Other, "Invalid Content-Length");
                    return Err(error);
                }
            };

            // Read `len` bytes from the reader
            let mut buffer = vec![0u8; len as usize];
            reader.read_exact(buffer.as_mut_slice())?;

            body = Some(buffer);
        }

        // Return the request
        Ok(Request {
            stream: reader.into_inner(),
            method: first_line[0].into(),
            target,
            version: first_line[2].into(),
            headers,
            body,
        })
    }

    pub fn method(&self) -> &str {
        &self.method[..]
    }

    pub fn target(&self) -> &Target {
        &self.target
    }

    pub fn version(&self) -> &str {
        &self.version[..]
    }

    pub fn header(&self, key: &str) -> Option<&str> {
        match self.headers.get(key) {
            Some(s) => Some(&s[..]),
            None => None,
        }
    }

    pub fn body(&self) -> Option<Vec<u8>> {
        self.body.clone()
    }

    pub fn respond(
        &mut self,
        status: &str,
        headers: HashMap<&str, String>,
        body: Vec<u8>,
    ) -> Result<usize> {
        // Format the first line
        let mut response = format!("{} {}\r\n", self.version, status);

        // Add the headers
        for (key, value) in headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        // Insert a blank line signaling the beginning of the body
        response.push_str("\r\n");

        // Convert to bytes
        let mut content = response.into_bytes();

        // Add the body
        content.extend(body);

        // Send
        self.stream.write(&content)?;
        self.stream.flush()?;

        Ok(content.len())
    }

    pub fn respond_with_file<'a>(
        &mut self,
        status: &str,
        mut headers: HashMap<&str, String>,
        filename: &str,
        filetype: &str,
    ) -> Result<usize> {
        let filename = format!("html/{}", filename);
        // Read in the file's bytes
        let bytes = fs::read(&filename[..])?;
        let len = bytes.len();

        // Add the content length and type headers
        headers.insert("Content-Type", filetype.to_string());
        headers.insert("Content-Length", format!("{}", len));

        self.respond(status, headers, bytes)
    }
}

pub fn parse_query(query: String) -> HashMap<String, Option<String>> {
    query
        .split("&")
        .map(|pair| {
            let pair: Vec<_> = pair.splitn(2, "=").collect();

            if pair.len() == 2 {
                (pair[0].to_string(), Some(pair[1].to_string()))
            } else {
                (pair[0].to_string(), None)
            }
        })
        .collect()
}
