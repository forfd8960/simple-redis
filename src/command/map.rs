use crate::{
    resp::{RespArray, RespFrame, RespNull},
    storage::memory::InMemStore,
};

use super::{extract_args, CommandError, CommandExecutor, Get, Set, RESP_OK};

impl CommandExecutor for Get {
    fn execute(self, store: &InMemStore) -> RespFrame {
        let value = store.get(&self.key);
        match value {
            Some(v) => v,
            None => RespFrame::Null(RespNull),
        }
    }
}

impl CommandExecutor for Set {
    fn execute(self, store: &InMemStore) -> RespFrame {
        store.set(self.key, self.value);
        RESP_OK.clone()
    }
}

impl TryFrom<RespArray> for Get {
    type Error = CommandError;
    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        let mut args = extract_args(value, 1)?.into_iter();
        match args.next() {
            Some(RespFrame::BulkString(key)) => Ok(Get {
                key: String::from_utf8(key.0)?,
            }),
            _ => Err(CommandError::InvalidArgument(
                "Get command must have a BulkString as the first argument".to_string(),
            )),
        }
    }
}
