use bytes::{BufMut, BytesMut};

use crate::dtls::common::{CipherSuite, ProtocolVersion};

pub enum HandshakeType {
    ClientHello = 1,
    HelloVerifyRequest = 3,
}

pub struct Random {
    pub gmt_unix_time: u32,
    pub random_bytes: [u8; 28],
}

pub enum CompressionMethod {
    Null,
}

pub struct Cookie(pub [u8; 32]);

pub struct ClientHello {
    pub client_version: ProtocolVersion,
    pub random: Random,
    pub session_id: u32,
    pub cookie: Cookie,
    pub cipher_suites: CipherSuite, // [u16; 16],
    pub compression_methods: Vec<CompressionMethod>,
}

impl ClientHello {
    pub fn write(&self, buf: &mut BytesMut) {
        buf.put_u8(self.client_version.major);
        buf.put_u8(self.client_version.minor);
    }
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
