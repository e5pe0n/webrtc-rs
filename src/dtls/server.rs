use std::{io, net::SocketAddr, sync::Arc};
use tokio::net::UdpSocket;

#[derive(Clone)]
pub struct Server {}

impl Server {
    pub fn new() -> Server {
        Server {}
    }

    pub async fn listen(self: Arc<Self>) -> io::Result<()> {
        let sock = UdpSocket::bind("0.0.0.0:8080".parse::<SocketAddr>().unwrap()).await?;

        tokio::spawn(async move {
            loop {
                let mut buf = vec![0; 1024];
                match sock.recv_from(&mut buf).await {
                    Ok((len, addr)) => {
                        buf.truncate(len);
                        let buf_clone = buf.clone();
                        println!("received from \"{}\": {:?}", addr, buf_clone);
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
