use super::{RespEncode, RespFrame};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RespSet(pub(crate) Vec<RespFrame>);

impl RespEncode for RespSet {
    fn encode(self) -> Vec<u8> {
        todo!()
    }
}
