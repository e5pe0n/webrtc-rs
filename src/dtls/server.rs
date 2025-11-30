use std::{io, net::SocketAddr};
use tokio::net::UdpSocket;

pub struct Server {}

impl Server {
    pub async fn listen(&self) -> io::Result<()> {
        let sock = UdpSocket::bind("0.0.0.0:8080".parse::<SocketAddr>().unwrap()).await?;

        tokio::spawn(async move {
            loop {
                let mut buf = vec![0; 10];
                match sock.recv_from(&mut buf).await {
                    Ok((len, addr)) => {
                        println!("received from \"{}\": {:?}", addr, buf);
                    }
                    Err(e) => {
                        eprintln!("received error: {}", e);
                    }
                }
            }
        });

        Ok(())
    }
}
