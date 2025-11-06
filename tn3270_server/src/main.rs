use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    // For now, just print a message when a client connects.
    println!("Client connected!");
    // Later, we'll add the Telnet and TN3270 logic here.
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2323").unwrap();
    println!("TN3270 server listening on port 2323");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
