pub mod array;
pub mod bool;
pub mod bulk_string;
pub mod double;
pub mod frame;
pub mod integer;
pub mod map;
pub mod null;
pub mod set;
pub mod simple_error;
pub mod simple_string;

use bytes::BytesMut;
use enum_dispatch::enum_dispatch;

#[allow(dead_code)]
const BUF_CAP: usize = 4096;
#[allow(dead_code)]
const CRLF: &[u8] = b"\r\n";
#[allow(dead_code)]
const CRLF_LEN: usize = CRLF.len();

pub use frame::*;
use thiserror::Error;

#[enum_dispatch]
pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

pub trait RespDecode: Sized {
    const PREFIX: &'static str;
    fn decode(buf: &mut BytesMut) -> anyhow::Result<Self, RespError>;
    fn expect_length(buf: &[u8]) -> anyhow::Result<usize, RespError>;
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum RespError {
    #[error("Invalid frame: {0}")]
    InvalidFrame(String),
    #[error("Invalid frametype: {0}")]
    InvalidFrameType(String),
    #[error("Invalid frame length: {0}")]
    InvalidFrameLength(isize),
    #[error("Frame is not complete")]
    NotComplete(isize),

    #[error("Parse error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Utf8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("Parse float error: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
}
