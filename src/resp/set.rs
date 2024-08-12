use std::ops::Deref;

use super::{RespDecode, RespEncode, RespFrame, BUF_CAP};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RespSet(pub(crate) Vec<RespFrame>);

// https://redis.io/docs/latest/develop/reference/protocol-spec/#sets
// RespSet encoding is: `~<number-of-elements>\r\n<element-1>...<element-n>`
impl RespEncode for RespSet {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(&format!("~{}\r\n", self.len()).into_bytes());

        for frame in self.0 {
            buf.extend_from_slice(&frame.encode())
        }
        buf
    }
}

impl RespDecode for RespSet {
    const PREFIX: &'static str = "~";
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, super::RespError> {
        Ok(RespSet::new(vec![]))
    }

    fn expect_length(buf: &[u8]) -> Result<usize, super::RespError> {
        Ok(0)
    }
}

impl RespSet {
    pub fn new(s: impl Into<Vec<RespFrame>>) -> Self {
        RespSet(s.into())
    }
}

impl Deref for RespSet {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
