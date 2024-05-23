use std::collections::BTreeMap;

use super::{RespEncode, RespFrame};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RespMap(pub(crate) BTreeMap<String, RespFrame>);

impl RespEncode for RespMap {
    fn encode(self) -> Vec<u8> {
        todo!()
    }
}
