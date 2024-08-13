use super::{extract_simple_frame_data, RespDecode, RespEncode, RespError, CRLF_LEN};

// :[<+|->]<value>\r\n
impl RespEncode for i64 {
    fn encode(self) -> Vec<u8> {
        let sign = if self < 0 { "" } else { "+" };
        format!(":{}{}\r\n", sign, self).into_bytes()
    }
}

impl RespDecode for i64 {
    const PREFIX: &'static str = ":";
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        let data = buf.split_to(end + CRLF_LEN);
        let s = String::from_utf8_lossy(&data[Self::PREFIX.len()..end]);
        Ok(s.parse()?)
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        Ok(end + CRLF_LEN)
    }
}

#[cfg(test)]
mod tests {
    use crate::resp::RespFrame;
    use anyhow::Result;
    use bytes::BytesMut;

    use super::*;

    #[test]
    fn test_encode_integer() {
        let frame: RespFrame = 1024.into();
        assert_eq!(frame.encode(), b":+1024\r\n");

        let frame1: RespFrame = (-99).into();
        assert_eq!(frame1.encode(), b":-99\r\n");
    }

    #[test]
    fn test_decode_integer() -> Result<()> {
        let data = b":+1024\r\n";
        let mut buf = BytesMut::new();
        buf.extend_from_slice(data);

        let num = i64::decode(&mut buf)?;
        assert_eq!(num, 1024);

        let data = b":-100\r\n";
        let mut buf = BytesMut::new();
        buf.extend_from_slice(data);

        let num = i64::decode(&mut buf)?;
        assert_eq!(num, -100);
        Ok(())
    }
}
