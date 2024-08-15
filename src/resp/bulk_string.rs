use std::ops::Deref;

use bytes::Buf;

use super::{extract_fixed_data, parse_length, RespDecode, RespEncode, RespError, CRLF_LEN};

const BULK_STRING_PREFIX: &'static str = "$";

// https://redis.io/docs/latest/develop/reference/protocol-spec/#bulk-strings
//$<length>\r\n<data>\r\n
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct BulkString(pub(crate) Vec<u8>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct RespNullBulkString;

impl BulkString {
    pub fn new(s: impl Into<Vec<u8>>) -> Self {
        BulkString(s.into())
    }
}

impl RespEncode for BulkString {
    fn encode(self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::with_capacity(self.len() + 16);
        buf.extend_from_slice(&format!("{}{}\r\n", BULK_STRING_PREFIX, self.len()).into_bytes());
        buf.extend_from_slice(&self);
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

// $3\r\nabc\r\n

impl RespDecode for BulkString {
    const PREFIX: &'static str = BULK_STRING_PREFIX;
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?; // 2, 3
        println!("{}, {}", end, len);
        let remain_data = &buf[end + CRLF_LEN..]; // abc\r\n
        println!("{:?}", String::from_utf8_lossy(remain_data));

        if remain_data.len() < len + CRLF_LEN {
            return Err(RespError::NotComplete);
        }

        buf.advance(end + CRLF_LEN);
        let data = buf.split_to(len + CRLF_LEN); // abc\r\n
        println!("{:?}", String::from_utf8_lossy(&data));

        Ok(BulkString::new(data[..len].to_vec())) // data[..len] => abc
    }
    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        Ok(end + CRLF_LEN + len + CRLF_LEN)
    }
}

impl RespEncode for RespNullBulkString {
    fn encode(self) -> Vec<u8> {
        b"-1\r\n".to_vec()
    }
}

impl RespDecode for RespNullBulkString {
    const PREFIX: &'static str = "$";
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, RespError> {
        extract_fixed_data(buf, "$-1\r\n", "NullBulkString")?;
        Ok(RespNullBulkString)
    }
    fn expect_length(_buf: &[u8]) -> Result<usize, RespError> {
        Ok(5)
    }
}

impl AsRef<[u8]> for BulkString {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Deref for BulkString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for BulkString {
    fn from(s: &str) -> Self {
        BulkString(s.as_bytes().to_vec())
    }
}

impl From<String> for BulkString {
    fn from(s: String) -> Self {
        BulkString(s.into_bytes())
    }
}

impl From<&[u8]> for BulkString {
    fn from(s: &[u8]) -> Self {
        BulkString(s.to_vec())
    }
}

impl<const N: usize> From<&[u8; N]> for BulkString {
    fn from(s: &[u8; N]) -> Self {
        BulkString(s.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use super::*;

    #[test]
    fn test_decode_works() -> anyhow::Result<()> {
        let mut buf = BytesMut::new();
        let data = b"$3\r\nabc\r\n".to_vec();
        buf.extend_from_slice(&data);

        let s = BulkString::decode(&mut buf)?;
        assert_eq!(s, BulkString::new(b"abc".to_vec()));
        Ok(())
    }
}
