use enum_dispatch::enum_dispatch;

use super::{
    BulkString, RespArray, RespMap, RespNullArray, RespNullBulkString, RespSet, SimpleError,
    SimpleString,
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
