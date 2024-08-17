use crate::{
    resp::{RespError, RespFrame},
    storage::memory::InMemStore,
};
use std::string::FromUtf8Error;
use thiserror::Error;

pub trait CommandExecutor {
    fn execute(self, store: &InMemStore) -> RespFrame;
}

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
