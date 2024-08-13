use enum_dispatch::enum_dispatch;

use super::{
    BulkString, RespArray, RespDecode, RespEncode, RespError, RespMap, RespNullArray,
    RespNullBulkString, RespSet, SimpleError, SimpleString,
};

#[enum_dispatch(RespEncode)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum RespFrame {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),
    Double(f64),
    Map(RespMap),
    Set(RespSet),
    BulkString(BulkString),
    NullBulkString(RespNullBulkString),
    Array(RespArray),
    NullArray(RespNullArray),
}

impl RespDecode for RespFrame {
    const PREFIX: &'static str = "";

    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, RespError> {
        todo!()
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        todo!()
    }
}
