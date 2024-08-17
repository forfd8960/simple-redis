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

// #[enum_dispatch(CommandExecutor)]
#[derive(Debug)]
pub enum Command {
    Get(Get),
    Set(Set),
    HGet(HGet),
    HSet(HGet),
    HGetAll(HGet),
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

impl TryFrom<RespArray> for Command {
    type Error = CommandError;
    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        todo!()
    }
}
