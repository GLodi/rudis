use std::io::{Read, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New client connected: {}", stream.peer_addr()?);
                let mut buffer = [0; 1024];
                loop {
                    let bytes_read = stream.read(&mut buffer)?;
                    if bytes_read == 0 {
                        println!("Client disconnected");
                        break;
                    }
                    stream.write(&buffer[..bytes_read])?;
                }
            }
            Err(e) => {
                println!("Error accepting client connection: {}", e);
            }
        }
    }

    Ok(())
}
