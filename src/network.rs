use bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

use crate::{
    resp::{RespDecode, RespEncode, RespError, RespFrame},
    storage::memory::InMemStore,
};

#[derive(Debug)]
struct RespFrameCodec;

#[derive(Debug)]
struct RedisRequest {
    frame: RespFrame,
    store: InMemStore,
}

#[derive(Debug)]
struct RedisResponse {
    frame: RespFrame,
}

impl Encoder<RespFrame> for RespFrameCodec {
    type Error = anyhow::Error;
    fn encode(&mut self, frame: RespFrame, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.extend_from_slice(&frame.encode());
        Ok(())
    }
}

impl Decoder for RespFrameCodec {
    type Item = RespFrame;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match RespFrame::decode(src) {
            Ok(frame) => Ok(Some(frame)),
            Err(RespError::NotComplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
