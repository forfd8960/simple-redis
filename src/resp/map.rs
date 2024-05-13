use std::collections::BTreeMap;

use super::RespFrame;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct RespMap(pub(crate) BTreeMap<String, RespFrame>);
