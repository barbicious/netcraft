use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;

    for stream in listener.incoming() {
        let mut stream = stream?;

        println!("Connection established");

        let handle = thread::spawn(move || {
            loop {
                let mut buffer = [0; 1024];

                let size = match stream.read(&mut buffer) {
                    Ok(0) => return Ok(()),
                    Ok(size) => size,
                    Err(e) => anyhow::bail!(e),
                };

                println!("Received: {:?}", String::from_utf8_lossy(&buffer[..size]));
            }
        });
    }

    Ok(())
}
