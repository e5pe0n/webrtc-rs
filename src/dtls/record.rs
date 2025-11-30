use crate::dtls::common::{ContentType, ProtocolVersion};

pub struct DtlsPlaintext {
    content_type: ContentType,
    epoch: u16,
    sequnce_number: u64, // u48
    length: u16,
    fragment: Vec<String>,
}

pub struct DtlsCompressed {
    content_type: ContentType,
    version: ProtocolVersion,
    epoch: u16,
    sequnce_number: u64, // u48
    length: u16,
    fragment: Vec<String>,
}

pub struct DtlsCipherText {
    content_type: ContentType,
    version: ProtocolVersion,
    epoch: u16,
    sequnce_number: u64, // u48
    length: u16,
    fragment: Vec<String>,
}
