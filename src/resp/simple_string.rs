use super::RespEncode;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SimpleString(pub(crate) String);

impl RespEncode for SimpleString {
    fn encode(self) -> Vec<u8> {
        format!("+{}\r\n", self.0).into_bytes()
    }
}
