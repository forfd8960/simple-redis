use super::{extract_fixed_data, RespDecode, RespEncode, RespError};

// https://redis.io/docs/latest/develop/reference/protocol-spec/#nulls
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct RespNull;

// _\r\n
impl RespEncode for RespNull {
    fn encode(self) -> Vec<u8> {
        b"_\r\n".to_vec()
    }
}

impl RespDecode for RespNull {
    const PREFIX: &'static str = "_";
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, RespError> {
        extract_fixed_data(buf, "_\r\n", "Null")?;
        Ok(RespNull)
    }
    fn expect_length(_buf: &[u8]) -> Result<usize, RespError> {
        Ok(3)
    }
}
