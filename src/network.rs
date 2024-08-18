use anyhow::Result;
use bytes::BytesMut;
use futures::SinkExt;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{Decoder, Encoder, Framed};
use tracing::info;

use crate::{
    command::{Command, CommandExecutor},
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

pub async fn stream_handler(stream: TcpStream, store: InMemStore) -> Result<()> {
    let mut framed = Framed::new(stream, RespFrameCodec);

    loop {
        match framed.next().await {
            Some(Ok(frame)) => {
                info!("received frame: {:?}", frame);
                let request = RedisRequest {
                    frame,
                    store: store.clone(),
                };

                let response = handle_request(request).await?;

                info!("sending response: {:?}", response.frame);
                framed.send(response.frame).await?;
            }
            Some(Err(e)) => return Err(e),
            None => return Ok(()),
        }
    }
}

async fn handle_request(request: RedisRequest) -> Result<RedisResponse> {
    let (frame, store) = (request.frame, request.store);
    let cmd = Command::try_from(frame)?;
    info!("Execute command: {:?}", cmd);

    let resp_frame = cmd.execute(&store);
    Ok(RedisResponse { frame: resp_frame })
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
