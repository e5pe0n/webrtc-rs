use crate::dtls::common::ProtocolVersion;

pub enum HandshakeType {
    ClientHello = 1,
    HelloVerifyRequest = 3,
}

struct Random {
    gmt_unix_time: u32,
    random_bytes: [u8; 28],
}

enum CompressionMethod {
    Null,
}

struct Cookie(pub [u8; 32]);

pub struct ClientHello {
    client_version: ProtocolVersion,
    random: Random,
    session_id: u32,
    cookie: Cookie,
    cipher_suites: [u16; 16],
    compression_method: Vec<CompressionMethod>,
}

pub struct HelloVerifyRequest {
    server_version: ProtocolVersion,
    cookie: Cookie,
}

pub enum HandshakeBody {
    ClientHello(ClientHello),
    HelloVerifyRequest(HelloVerifyRequest),
}

pub struct HandshakeHeader {
    msg_type: HandshakeType,
    length: u32,          // u24,
    message_seq: u16,     // u16
    fragment_offset: u32, // u24,
    fragment_length: u32, // u24,
}

pub struct Handshake {
    header: HandshakeHeader,
    body: HandshakeBody,
}
