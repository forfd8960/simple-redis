use super::{extract_simple_frame_data, RespDecode, RespEncode, RespError, BUF_CAP, CRLF_LEN};

const DOUBLE_PREFIX: &'static str = ",";

// https://redis.io/docs/latest/develop/reference/protocol-spec/#doubles
// ,[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n
impl RespEncode for f64 {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        let ret = if self.abs() > 1e+8 || self.abs() < 1e-8 {
            format!("{}{:+e}\r\n", DOUBLE_PREFIX, self)
        } else {
            let sign = if self < 0.0 { "" } else { "+" };
            format!("{}{}{}\r\n", DOUBLE_PREFIX, sign, self)
        };

        buf.extend_from_slice(&ret.into_bytes());
        buf
    }
}

impl RespDecode for f64 {
    const PREFIX: &'static str = DOUBLE_PREFIX;
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
    fn test_encode_double() {
        let frame: RespFrame = 1000.24.into();
        assert_eq!(frame.encode(), b",+1000.24\r\n");

        let frame1: RespFrame = (-8.9999).into();
        assert_eq!(frame1.encode(), b",-8.9999\r\n");
    }

    #[test]
    fn test_decode_double() -> Result<()> {
        let data = b",+1.024\r\n";
        let mut buf = BytesMut::new();
        buf.extend_from_slice(data);

        let num = f64::decode(&mut buf)?;
        assert_eq!(num, 1.024);

        let data1 = b",-9.88\r\n";
        let mut buf = BytesMut::new();
        buf.extend_from_slice(data1);

        let num1 = f64::decode(&mut buf)?;
        assert_eq!(num1, -9.88);
        Ok(())
    }
}
