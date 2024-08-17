pub mod map;

use crate::{
    resp::{RespArray, RespError, RespFrame, SimpleString},
    storage::memory::InMemStore,
};
use enum_dispatch::enum_dispatch;
use lazy_static::lazy_static;
use std::string::FromUtf8Error;
use thiserror::Error;

lazy_static! {
    static ref RESP_OK: RespFrame = SimpleString::new("OK").into();
}

#[enum_dispatch]
pub trait CommandExecutor {
    fn execute(self, store: &InMemStore) -> RespFrame;
}

#[enum_dispatch(CommandExecutor)]
#[derive(Debug)]
pub enum Command {
    Get(Get),
    Set(Set),
    // HGet(HGet),
    // HSet(HGet),
    // HGetAll(HGet),
    Unknown(Unknown),
}

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Invalid Command: {0}")]
    InvalidCommand(String),
    #[error("Invalid Argument: {0}")]
    InvalidArgument(String),

    #[error("{0}")]
    RespError(#[from] RespError),
    #[error("Utf8 error: {0}")]
    Utf8Error(#[from] FromUtf8Error),
}

#[derive(Debug)]
pub struct Get {
    key: String,
}

#[derive(Debug)]
pub struct Set {
    key: String,
    value: RespFrame,
}

#[derive(Debug)]
pub struct HGet {
    key: String,
    field: String,
}

#[derive(Debug)]
pub struct HSet {
    key: String,
    field: String,
    value: RespFrame,
}

#[derive(Debug)]
pub struct HGetAll {
    key: String,
}

#[derive(Debug)]
pub struct Unknown;

impl TryFrom<RespFrame> for Command {
    type Error = CommandError;
    fn try_from(value: RespFrame) -> Result<Self, Self::Error> {
        match value {
            RespFrame::Array(arr) => arr.try_into(),
            _ => Err(CommandError::InvalidCommand(
                "Command must be an array".to_string(),
            )),
        }
    }
}

impl TryFrom<RespArray> for Command {
    type Error = CommandError;
    fn try_from(v: RespArray) -> Result<Self, Self::Error> {
        match v.first() {
            Some(RespFrame::BulkString(ref cmd)) => match cmd.as_ref() {
                b"get" => Ok(Get::try_from(v)?.into()),
                _ => Ok(Unknown.into()),
            },
            _ => Err(CommandError::InvalidCommand(
                "Command must have a BulkString as the first argument".to_string(),
            )),
        }
    }
}

impl CommandExecutor for Unknown {
    fn execute(self, _: &InMemStore) -> RespFrame {
        RESP_OK.clone()
    }
}

fn extract_args(val: RespArray, start: usize) -> Result<Vec<RespFrame>, CommandError> {
    Ok(val.0.into_iter().skip(start).collect::<Vec<RespFrame>>())
}
