mod dtls;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::dtls::server::Server;

    use super::*;

    #[tokio::test]
    async fn test_listener() -> tokio::io::Result<()> {
        let server = Server {};
        server.listen().await?;
        Ok(())
    }
}
