use std::{ops::Deref, sync::Arc};

use dashmap::DashMap;

use crate::resp::RespFrame;

#[derive(Debug, Clone)]
pub struct InMemStore(Arc<InMemStoreInner>);

#[derive(Debug)]
pub struct InMemStoreInner {
    pub(crate) map: DashMap<String, RespFrame>,
    pub(crate) hmap: DashMap<String, DashMap<String, RespFrame>>,
}

impl Deref for InMemStore {
    type Target = InMemStoreInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for InMemStore {
    fn default() -> Self {
        Self(Arc::new(InMemStoreInner::default()))
    }
}

impl Default for InMemStoreInner {
    fn default() -> Self {
        Self {
            map: DashMap::new(),
            hmap: DashMap::new(),
        }
    }
}
