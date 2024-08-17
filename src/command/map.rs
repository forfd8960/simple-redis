use crate::{
    resp::{RespFrame, RespNull},
    storage::memory::InMemStore,
};

use super::{CommandExecutor, Get, Set, RESP_OK};

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
