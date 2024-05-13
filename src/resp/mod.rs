pub mod array;
pub mod bool;
pub mod double;
pub mod frame;
pub mod integer;
pub mod map;
pub mod null;
pub mod set;
pub mod simple_error;
pub mod simple_string;

#[allow(dead_code)]
const BUF_CAP: usize = 4096;
#[allow(dead_code)]
const CRLF: &[u8] = b"\r\n";
#[allow(dead_code)]
const CRLF_LEN: usize = CRLF.len();

pub use frame::*;
