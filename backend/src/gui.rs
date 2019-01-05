use crate::util::Config;

use std::process::Command;
use std::net::UdpSocket;

use std::io::prelude::*;
use std::net::TcpListener;

pub fn begin(mut config: Config) {
/*    // Start the GUI
    let gui_proc = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "../frontend/gradlew run"])
            .output()
            .expect("failed to execute process");
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("../frontend/gradlew run")
            .output()
            .expect("failed to execute process");
    };*/

    // Build the address
    let address = if let Some(port) = config.port {
        // Try to use the port specified
        format!("127.0.0.1:{}", port)
    } else {
        // Let the OS assign a port
        String::from("127.0.0.1:0")
    };

    println!("Binding to address {}", address);

    // Connect via a socket on the given port
    // let mut socket = UdpSocket::bind(address).unwrap();
    let mut listener = TcpListener::bind(address).unwrap();

    // Send 
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => println!("connection!"),
            Err(_) => println!("err")
        }
    }

    // TODO: begin comm loop
}
