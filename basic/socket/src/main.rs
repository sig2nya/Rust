use tokio::net::{
    TcpListener,
    TcpStream
};
use tokio::io::{
    AsyncReadExt,
    AsyncWriteExt
};

#[tokio::main]
async fn main() -> io::Result<()> {
    /*
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    match stream.read(&mut buf) {
        Ok(n) if n == 0 => {
            println!("client disconnected");
            // Client sended FIN(EOF)
        }
        Ok(n) => {
            println!("read {} bytes", n);
        }
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
            println!("no data yet");
        }
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
            println!("timeout - possible network delay or dead peer");
        }
        Err(e) => {
            println!("unexpected error: {}", e);
        }
    }
    */
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    loop {
        let (mut socket, addr) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0u8, 1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => socket.write_all(&buf[..n]).await.unwrap(),
                    Err(e) => { eprintln!("error: {}", e); break; }
                }
            }
        });
    }
}
