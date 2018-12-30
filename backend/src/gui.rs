use crate::util::Config;
use crate::threadpool::ThreadPool;

use std::{
    collections::HashMap,
    net::{TcpStream, TcpListener},
    io::{self, prelude::*},
    sync::mpsc,
    fs
};

pub fn begin(config: Config) {
    // Build the address
    let listener = if let Some(port) = config.port {
        // Try to use the port specified
        TcpListener::bind(&format!("127.0.0.1:{}", port)).unwrap()
    } else {
        // Let the OS assign a port
        TcpListener::bind("127.0.0.1:0").unwrap()
    };

    // Put in non-blocking mode
    listener.set_nonblocking(true).unwrap();

    println!("Listening on port {}.", listener.local_addr().unwrap().port());

    // Create a channel so the threads can tell the server to stop
    let (tx_stop, rx_stop) = mpsc::channel();
    
    // Create a new thread pool
    let pool = ThreadPool::new(10, tx_stop);

    // Listen for incoming connections
    loop {
        let stream = listener.accept();

        // Check the stop receiver
        match rx_stop.try_recv() {
            Err(mpsc::TryRecvError::Empty) => (),
            _ => break
        };

        // Extract the stream
        let stream = match stream {
            Ok(stream) => stream.0,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue
            },
            Err(e) => {
                println!("Error: {:?}", e);
                continue
            }
        };

        // Send the connection to a thread
        pool.execute(|stop_sender: mpsc::Sender<()>| {
            if handle_connection(stream) {
                stop_sender.send(()).unwrap();
            }
        });
    }
}

// Handle the connection, return true to exit
fn handle_connection(mut stream: TcpStream) -> bool {
    let mut bytes = Vec::new();

    loop {
        // Read the request to buffer
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();

        bytes.extend_from_slice(&buffer);

        if buffer[511] == 0 {
            break
        }
    }

    // Separate the method and request path
    let request = String::from_utf8_lossy(&bytes[..]);

    println!("--{}--", request);

    if bytes.len() == 0 || bytes[0] == 0 {
        return false;
    }

    let parts: Vec<_> = request.split_whitespace().take(2).collect();

    assert!(parts.len() == 2, format!("Request: {} (len: {})", request, bytes.len()));

    let method = parts[0];

    // Extract the query
    let query: Vec<_> = parts[1].split("?").collect();

    let path = query[0];

    // Extract query items
    let query = if query.len() == 1 {
        vec![]
    } else {
        query[1].split("&").collect::<Vec<_>>()
    };

    // Convert query vector to a hash
    let query = {
        let mut query_hashmap: HashMap<String, Option<String>> = HashMap::new();

        // Split items into key and value and add into hashmap
        for item in query {
            let parts: Vec<_> = item.split("=").collect();

            let key = parts[0].to_string();
            let value = if parts.len() == 1 {
                None
            } else {
                Some(parts[1].to_string())
            };

            query_hashmap.insert(key, value);
        }

        query_hashmap
    };

    let mut contents = String::new().into_bytes();
    let mut quit = false;

    println!("Method: {}\nPath: {}\nQuery: {:?}", method, path, query);

    // Figure out what to do
    if path == "/" && method == "POST" && query.contains_key("action") {
        let action = query.get("action").unwrap();

        if let Some(action) = action {
            // Do the given action
            if action == "quit" {
                // Send back the quit message
                let html = fs::read_to_string("html/terminated.html").unwrap();

                contents = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}", html.len(), html).into_bytes();
                quit = true;
            }
        }
    } else if path == "/style.css" && method == "GET" {
        let css = fs::read_to_string("html/style.css").unwrap();

        contents = format!("HTTP/1.1\r\n\r\n{}", css).into_bytes();
    } else if path == "/sample.png" && method == "GET" {
        let bytes = fs::read("html/sample.png").unwrap();
        let len = bytes.len();

        contents = format!("HTTP/1.1\r\nContent-Type:image/png\r\nContent-Length:{}\r\n\r\n", len).into_bytes();
        contents.extend(bytes);
    }

    // If we've reached here, and contents is empty, just return the normal dashboard page
    if contents == b"" {
        // Send dashboard
        let html = fs::read_to_string("html/dashboard.html").unwrap();

        contents = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}", html.len(), html).into_bytes();
    }

    // Send the response
    stream.write(&contents).unwrap();
    stream.flush().unwrap();

    quit
}
