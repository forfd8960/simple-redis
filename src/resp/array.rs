use super::{RespEncode, RespFrame, BUF_CAP};

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

// NullArray: `*-1\r\n`
impl RespEncode for RespNullArray {
    fn encode(self) -> Vec<u8> {
        b"*-1\r\n".to_vec()
    }
}
