use super::{RespEncode, BUF_CAP};

impl RespEncode for f64 {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        let ret = if self.abs() > 1e+8 || self.abs() < 1e-8 {
            format!(",{:+e}\r\n", self)
        } else {
            let sign = if self < 0.0 { "" } else { "+" };
            format!(",{}{}\r\n", sign, self)
        };

        buf.extend_from_slice(&ret.into_bytes());
        buf
    }
}
