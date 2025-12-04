mod dtls;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use std::{
        net::SocketAddr,
        sync::Arc,
        time::{SystemTime, UNIX_EPOCH},
    };

    use crate::dtls::{
        common::{CipherSuite, ProtocolVersion},
        handshake::{ClientHello, CompressionMethod, Cookie, Random},
        server::Server,
    };

    use tokio::{net::UdpSocket, sync::oneshot};

    #[tokio::test]
    async fn test_listener() -> tokio::io::Result<()> {
        let server = Arc::new(Server::new());
        server.listen().await?;

        let client_hello = ClientHello {
            client_version: ProtocolVersion { major: 1, minor: 0 },
            random: Random {
                gmt_unix_time: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as u32,
                random_bytes: [0; 28],
            },
            session_id: 0,
            cookie: Cookie([0; 32]),
            cipher_suites: CipherSuite::TLS_NULL_WITH_NULL_NULL,
            compression_methods: vec![CompressionMethod::Null],
        };

        let buf: [u8; 4] = [1, 2, 3, 4];

        let sock = UdpSocket::bind("0.0.0.0:0").await?;
        let len = sock.send_to(&buf, "0.0.0.0:8080").await?;

        Ok(())
    }
}
