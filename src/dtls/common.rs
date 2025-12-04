pub enum ContentType {
    Handshake = 22,
}
pub struct ProtocolVersion {
    pub major: u8,
    pub minor: u8,
}

pub enum CipherSuite {
    TLS_NULL_WITH_NULL_NULL = 0x0000,
}
