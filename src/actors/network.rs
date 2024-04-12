use async_trait::async_trait;
use bytes::BytesMut;
use prost::Message;
use tokio::net::TcpListener;
use tokio::sync::mpsc::Receiver;
use tracing::{error, info, trace};

use crate::actors::{Actor, ActorHandle};
use crate::actors::scene::ScenesActor;
use crate::errors::ErrorCode;
use crate::protos::global::ErrorResponse;

#[derive(Debug)]
pub struct NetworkActor {
    receiver: Receiver<crate::Message<String>>,
}

#[async_trait]
impl Actor for NetworkActor {
    type Msg = String;

    async fn handle(&self, _: crate::Message<Self::Msg>) {
        trace!("Creating TcpListener...");
        let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
        trace!("TcpListener created!");
        while let Ok((socket, _)) = listener.accept().await {
            info!("Received a request from {:?}", match socket.peer_addr() {
                Ok(addr) => addr,
                Err(e) => {
                    error!("{:?}", e);
                    let mut frame = BytesMut::with_capacity(64);
                    ErrorResponse {
                        error_code: ErrorCode::NetworkError as u32
                    }.encode(&mut frame).unwrap();
                    return
                }
            });
            let scenes_handle = ActorHandle::new::<ScenesActor>();
            scenes_handle.send(crate::Message::new(socket)).await.unwrap();
        }
    }

    async fn run(&mut self) {
        while let Some(message) = self.receiver.recv().await {
            self.handle(message).await;
        }
    }

    fn new(receiver: Receiver<crate::Message<Self::Msg>>) -> Self {
        Self { receiver }
    }
}
