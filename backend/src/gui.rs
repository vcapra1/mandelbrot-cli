use crate::util::Config;
use crate::threadpool::ThreadPool;
use crate::http;

use std::{
    net::{TcpStream, TcpListener},
    io,
    collections::HashMap,
    sync::mpsc,
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
fn handle_connection(stream: TcpStream) -> bool {
    let mut request = match http::Request::new(stream) {
        Ok(req) => req,
        Err(_) => return false
    };

    // List of valid files that can be directly accessed and returned
    let valid_files: HashMap<_, _> = vec![
        ("/style.css", "text/css"), 
        ("/sample.png", "image/png")
    ].into_iter().map(|pair| {
         (pair.0.to_string(), pair.1.to_string())
    }).collect();

    // Decide what to do based on request method
    let (exit, bytes_sent) = if request.method() == "GET" {
        if valid_files.contains_key(&request.target().path) {
            // Serve that file directly
            let filename = &request.target().path.clone()[..];
            let filetype = valid_files.get(&request.target().path).unwrap();
            (false, request.respond_with_file("200 OK", HashMap::new(), filename, filetype).unwrap_or(0))
        } else if request.target().path == "/" {
            // Serve the dashboard
            let filename = "dashboard.html";
            let filetype = "text/html";
            (false, request.respond_with_file("200 OK", HashMap::new(), filename, filetype).unwrap_or(0))
        } else {
            // Respond with 404 error
            (false, request.respond("404 Not Found", HashMap::new(), Vec::new()).unwrap_or(0))
        }
    } else if request.method() == "POST" {
        if request.target().path == "/" {
            // Get body
            let body = match request.body() {
                Some(data) => {
                    // Convert to string
                    if request.header("Content-Type") == Some("application/x-www-form-urlencoded") {
                        (String::from_utf8_lossy(data.as_slice())).to_string()
                    } else {
                        String::new()
                    }
                },
                None => String::new()
            };

            // Read query and do something
            let data = http::parse_query(body);

            let mut filename = "dashboard.html";
            let mut q = false;

            if data.contains_key("action") {
                // Get the action
                let action = data.get("action").unwrap();

                if let Some(action) = action {
                    if action == "Quit" {
                        filename = "terminated.html";
                        q = true
                    }
                }
            }

            // Serve the html file
            let filetype = "text/html";
            (q, request.respond_with_file("200 OK", HashMap::new(), filename, filetype).unwrap_or(0))
        } else {
            // Respond with 404 error
            (false, request.respond("404 Not Found", HashMap::new(), Vec::new()).unwrap_or(0))
        }
    } else {
        // Respond with 405 Method Not Allowed
        (false, request.respond("405 Method Not Allowed", HashMap::new(), Vec::new()).unwrap_or(0))
    };

    exit
}
