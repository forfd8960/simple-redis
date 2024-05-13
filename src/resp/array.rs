use super::RespFrame;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct RespArray(pub(crate) Vec<RespFrame>);
