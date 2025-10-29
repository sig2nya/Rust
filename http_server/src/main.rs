use std::net::TcpListener;
use std::io::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        
        let response = "HTTP/1.1 200 OK\r\nContent_Length: 13\r\n\r\nHello, world!";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
