use std::ops::Deref;

use crate::resp::extract_simple_frame_data;

use super::{RespDecode, RespEncode, RespError, CRLF_LEN};

// Simple strings are encoded as a plus (+) character, followed by a string. The string mustn't contain a CR (\r) or LF (\n) character and is terminated by CRLF (i.e., \r\n).
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SimpleString(pub(crate) String);

impl SimpleString {
    pub fn new(s: impl Into<String>) -> Self {
        SimpleString(s.into())
    }
}

impl RespEncode for SimpleString {
    fn encode(self) -> Vec<u8> {
        format!("+{}\r\n", self.0).into_bytes()
    }
}

impl RespDecode for SimpleString {
    const PREFIX: &'static str = "+";
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        println!("end: {}", end);
        println!("end + CRLF_LEN): {}", end + CRLF_LEN);

        let data = buf.split_to(end + CRLF_LEN);
        let s = String::from_utf8_lossy(&data[Self::PREFIX.len()..end]);
        Ok(SimpleString(s.to_string()))
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        Ok(end + CRLF_LEN)
    }
}

impl From<&str> for SimpleString {
    fn from(value: &str) -> Self {
        SimpleString(value.to_string())
    }
}

impl AsRef<str> for SimpleString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for SimpleString {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::resp::RespFrame;
    use anyhow::Result;
    use bytes::BytesMut;

    use super::*;

    #[test]
    fn test_simple_string() {
        let frame: RespFrame = SimpleString::new("OK").into();
        let encoded = frame.encode();
        assert_eq!(encoded, b"+OK\r\n".to_vec());

        let frame1: RespFrame = SimpleString::new("Hello,World!").into();
        let encoded = frame1.encode();
        assert_eq!(encoded, b"+Hello,World!\r\n".to_vec());
    }

    #[test]
    fn test_simple_string_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"+OK\r\n");

        println!("b+: {:?}", b"+".to_vec());

        let s = SimpleString::decode(&mut buf)?;
        assert_eq!(s, SimpleString::new("OK"));
        Ok(())
    }
}
