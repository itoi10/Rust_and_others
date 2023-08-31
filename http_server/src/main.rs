use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    // TCP listener
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    let response =
    // GET / HTTP/1.1
    if buffer.starts_with(get) {
        let mut file = File::open("index.html").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        format!("HTTP/1.1 200 OK\r\n\r\n{}", contents)
    }
    // other
    else {
        format!("HTTP/1.1 404 NOT FOUND\r\n\r\n")
    };

    println!("Response: {}", response);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
