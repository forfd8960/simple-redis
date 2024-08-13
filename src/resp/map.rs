use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use super::{RespDecode, RespEncode, RespError, RespFrame, SimpleString, BUF_CAP};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RespMap(pub(crate) BTreeMap<String, RespFrame>);

// %<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>
impl RespEncode for RespMap {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(&format!("%{}\r\n", self.len()).into_bytes());

        for (key, value) in self.0 {
            buf.extend_from_slice(&SimpleString::new(key).encode());
            buf.extend_from_slice(&value.encode());
        }
        buf
    }
}

impl RespDecode for RespMap {
    const PREFIX: &'static str = "%";
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, RespError> {
        todo!()
    }
    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        todo!()
    }
}

impl RespMap {
    pub fn new() -> Self {
        RespMap(BTreeMap::new())
    }
}

impl Default for RespMap {
    fn default() -> Self {
        RespMap::new()
    }
}

impl Deref for RespMap {
    type Target = BTreeMap<String, RespFrame>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RespMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
