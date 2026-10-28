
use std::collections::HashSet;
use std::net::{SocketAddr, UdpSocket};

fn main() -> anyhow::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:3000")?;
    let mut clients: HashSet<SocketAddr> = HashSet::new();
    let mut buffer = [0u8; 1024];

    loop {
        let (bytes_read, addr) = socket.recv_from(&mut buffer)?;

        if bytes_read != 8 {
            continue;
        }

        clients.insert(addr);

        let pos: &[f32; 2] = bytemuck::from_bytes(&buffer[..8]);

        println!(
            "Received from {addr}: {}x{}",
            pos[0], pos[1]
        );

        for client in clients.iter().filter(|&&client| client != addr) {
            socket.send_to(&buffer[..bytes_read], client)?;
        }
    }
}
