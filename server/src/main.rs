use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    //println!("received request: {}", String::from_utf8_lossy(&buffer[..]));

    let (status_line, filename) =
    if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else if buffer.starts_with(b"GET /sleep HTTP/1.1\r\n") {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "error.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
