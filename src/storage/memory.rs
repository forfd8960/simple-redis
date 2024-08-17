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

impl InMemStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &str) -> Option<RespFrame> {
        self.map.get(key).map(|v| v.value().clone())
    }

    pub fn set(&self, key: String, value: RespFrame) {
        self.map.insert(key, value);
    }

    pub fn hget(&self, key: &str, field: &str) -> Option<RespFrame> {
        self.hmap
            .get(key)
            .and_then(|v| v.get(field).map(|v| v.value().clone()))
    }
    pub fn hset(&self, key: String, field: String, value: RespFrame) {
        let hmap = self.hmap.entry(key).or_default();
        hmap.insert(field, value);
    }

    pub fn hgetall(&self, key: &str) -> Option<DashMap<String, RespFrame>> {
        self.hmap.get(key).map(|v| v.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get() {
        let store = InMemStore::new();
        store.set("foo".to_string(), RespFrame::SimpleString("Hello".into()));
        assert_eq!(
            store.get("foo"),
            Some(RespFrame::SimpleString("Hello".into()))
        );
    }

    #[test]
    fn test_hset_hget() {
        let store = InMemStore::new();
        store.hset(
            "price".to_string(),
            "Mac".to_string(),
            RespFrame::Integer(3000),
        );
        let value = store.hget("price", "Mac");
        assert_eq!(value, Some(RespFrame::Integer(3000)));
    }

    #[test]
    fn test_hgetall() {
        let store = InMemStore::new();
        store.hset(
            "blog".to_string(),
            "author".to_string(),
            RespFrame::SimpleString("Alex".into()),
        );
        store.hset(
            "blog".to_string(),
            "content".to_string(),
            RespFrame::SimpleString("this is a blog".into()),
        );
        store.hset(
            "blog".to_string(),
            "create_time".to_string(),
            RespFrame::SimpleString("2024-08-17".into()),
        );

        let blog_data = store.hgetall("blog").unwrap();

        assert_eq!(blog_data.len(), 3);
        assert_eq!(
            blog_data.get("author").unwrap().value(),
            &RespFrame::SimpleString("Alex".into())
        );
        assert_eq!(
            blog_data.get("content").unwrap().value(),
            &RespFrame::SimpleString("this is a blog".into())
        );
        assert_eq!(
            blog_data.get("create_time").unwrap().value(),
            &RespFrame::SimpleString("2024-08-17".into())
        );
    }
}
