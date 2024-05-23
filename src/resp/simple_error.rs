use super::RespEncode;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SimpleError(pub(crate) String);

impl RespEncode for SimpleError {
    fn encode(self) -> Vec<u8> {
        format!("-{}\r\n", self.0).into_bytes()
    }
}
