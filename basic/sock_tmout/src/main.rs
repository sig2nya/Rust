use std::time::Duration;
use std::net::{TcpListener};
use std::io::{Read};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let (mut stream, _) = listener.accept().unwrap();
    stream.set_read_timeout(Some(Duration::from_secs(10))).unwrap();

    let mut buf = [0; 1024];
    match stream.read(&mut buf) {
        Ok(0) => println!("client closed connection"),
        Ok(n) => println!("got {} bytes", n),
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            println!("read would block");
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::TimeOut => {
            println!("timeout - no data from client");
        }
        Err(e) => println!("error : {}", e),
    }
}
