use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, UdpSocket};
use std::thread;

fn main() -> anyhow::Result<()> {
    let listener = UdpSocket::bind("127.0.0.1:3000")?;

    let mut clients = HashMap::new();

    loop {
        let mut buffer = [0; 1024];
        let (bytes_read, addr) = listener.recv_from(&mut buffer)?;

        clients.insert(addr, thread::spawn(move || {
            println!("Received: {:?}", String::from_utf8_lossy(&buffer[..bytes_read]));
        }));

        println!("Connection established");
    }

    Ok(())
}
