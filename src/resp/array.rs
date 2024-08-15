use std::ops::Deref;

use bytes::{Buf, BytesMut};

use super::{
    calculate_total_length, extract_fixed_data, parse_length, RespDecode, RespEncode, RespError,
    RespFrame, BUF_CAP, CRLF_LEN,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RespArray(pub(crate) Vec<RespFrame>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct RespNullArray;

// RespArray:
// https://redis.io/docs/latest/develop/reference/protocol-spec/#arrays
// `*<number-of-elements>\r\n<element-1>...<element-n>`
impl RespEncode for RespArray {
    fn encode(self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::with_capacity(BUF_CAP);

        buf.extend_from_slice(&format!("*{}\r\n", self.0.len()).into_bytes());

        for frame in self.0 {
            buf.extend_from_slice(&frame.encode())
        }
        buf
    }
}

impl RespDecode for RespArray {
    const PREFIX: &'static str = "*";
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        let total_len = calculate_total_length(buf, end, len, Self::PREFIX)?;
        if buf.len() < total_len {
            return Err(RespError::NotComplete);
        }

        buf.advance(end + CRLF_LEN);

        let mut frames = Vec::with_capacity(len);
        for _ in 0..len {
            let frame = RespFrame::decode(buf)?;
            frames.push(frame);
        }

        Ok(RespArray::new(frames))
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        calculate_total_length(buf, end, len, Self::PREFIX)
    }
}

// NullArray: `*-1\r\n`
impl RespEncode for RespNullArray {
    fn encode(self) -> Vec<u8> {
        b"*-1\r\n".to_vec()
    }
}

impl RespDecode for RespNullArray {
    const PREFIX: &'static str = "*";
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, RespError> {
        extract_fixed_data(buf, "*-1\r\n", "NullArray")?;
        Ok(RespNullArray)
    }
    fn expect_length(_buf: &[u8]) -> Result<usize, RespError> {
        Ok(5)
    }
}

impl RespArray {
    pub fn new(arr: impl Into<Vec<RespFrame>>) -> Self {
        RespArray(arr.into())
    }
}

impl Deref for RespArray {
    type Target = Vec<RespFrame>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<RespFrame>> for RespArray {
    fn from(value: Vec<RespFrame>) -> Self {
        RespArray(value)
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use crate::resp::BulkString;

    use super::*;

    #[test]
    fn test_array_encode() {
        let frame: RespFrame = RespArray::new(vec![
            BulkString::new("A".to_string()).into(),
            BulkString::new("B".to_string()).into(),
            BulkString::new("C".to_string()).into(),
        ])
        .into();

        assert_eq!(&frame.encode(), b"*3\r\n$1\r\nA\r\n$1\r\nB\r\n$1\r\nC\r\n");
    }

    #[test]
    fn decode_array() -> anyhow::Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n");

        let array = RespArray::decode(&mut buf)?;
        assert_eq!(
            array,
            RespArray::new(vec![
                BulkString::new("foo".to_string()).into(),
                BulkString::new("bar".to_string()).into(),
            ])
        );
        Ok(())
    }
}
