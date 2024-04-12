use std::collections::HashMap;

use async_trait::async_trait;
use bytes::BytesMut;
use prost::Message;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::Receiver;
use tracing::{debug, error, info, trace};

use crate::actors::{Actor, ActorHandle};
use crate::errors::ErrorCode;
use crate::protos::global::ErrorResponse;

use crate::protos::scenes::{SelectSceneRequest, SelectSceneResponse};

#[derive(Debug)]
pub enum Scene {
    SceneA
}

impl Scene {
    pub fn new_handle(&self) -> ActorHandle<TcpStream> {
        match self {
            Scene::SceneA => ActorHandle::new::<SceneAActor>()
        }
    }
}

#[derive(Debug)]
pub struct ScenesActor {
    receiver: Receiver<crate::actors::Message<TcpStream>>,
    scenes: HashMap<u32, Scene>
}

#[async_trait]
impl Actor for ScenesActor {
    type Msg = TcpStream;

    async fn handle(&self, mut message: crate::actors::Message<Self::Msg>) {
        let mut frame = BytesMut::with_capacity(64);
        info!("Scenes Actor is handling request from {:?}...", match message.data.peer_addr() {
            Ok(addr) => addr,
            Err(e) => {
                error!("{:?}", e);
                ErrorResponse {
                    error_code: ErrorCode::NetworkError as u32
                }.encode(&mut frame).unwrap();
                return
            }
        });
        trace!("Reading SelectSceneRequest frame from TcpStream...");
        if let Err(e) = message.data.read_buf(&mut frame).await {
            error!("{:?}", e);
            return
        }
        trace!("Finished reading SelectSceneRequest!");
        trace!("Decoding request into struct...");
        let request = match SelectSceneRequest::decode(&*frame) {
            Ok(request) => request,
            Err(e) => {
                error!("{:?}", e);
                ErrorResponse {
                    error_code: ErrorCode::DecodeProtoFailed as u32
                }.encode(&mut frame).unwrap();
                return
            }
        };
        trace!("Finished decoding request into struct!");
        trace!("Finding scene with request...");
        let handle = match self.scenes.get(&request.scene_id) {
            None => {
                error!("Scene not exist!");
                ErrorResponse {
                    error_code: ErrorCode::SceneNotExist as u32
                }.encode(&mut frame).unwrap();
                return;
            },
            Some(scene) => {
                debug!("Finished finding scene with request!");
                scene.new_handle()
            }
        };
        if let Err(e) = handle.send(message).await {
            error!("{:?}", e);
            ErrorResponse {
                error_code: ErrorCode::ChannelClosed as u32
            }.encode(&mut frame).unwrap();
        }
    }

    async fn run(&mut self) {
        while let Some(message) = self.receiver.recv().await {
            self.handle(message).await;
        }
    }

    fn new(receiver: Receiver<crate::actors::Message<Self::Msg>>) -> Self {
        let mut scenes = HashMap::new();
        scenes.insert(1, Scene::SceneA);
        Self { receiver, scenes }
    }
}

#[derive(Debug)]
pub struct SceneAActor {
    receiver: Receiver<crate::actors::Message<TcpStream>>
}

#[async_trait]
impl Actor for SceneAActor {
    type Msg = TcpStream;

    async fn handle(&self, mut message: crate::actors::Message<Self::Msg>) {
        let mut frame = BytesMut::with_capacity(64);
        info!("Scene A Actor is handling request from {:?}...", match message.data.peer_addr() {
            Ok(addr) => addr,
            Err(e) => {
                ErrorResponse {
                    error_code: ErrorCode::NetworkError as u32
                }.encode(&mut frame).unwrap();
                error!("{:?}", e);
                return
            }
        });

        trace!("Encoding SelectSceneResponse to binary...");
        SelectSceneResponse {
            success: true,
        }.encode(&mut frame).unwrap();
        trace!("Finished encoding SelectSceneResponse to binary...");

        trace!("Writing binary to frame...");
        message.data.write_buf(&mut frame).await.unwrap();
        trace!("Finished binary to frame...");

        frame.clear();

        trace!("Reading client request...");
        message.data.read_buf(&mut frame).await.unwrap();
        debug!("{:?}", frame);
    }

    async fn run(&mut self) {
        while let Some(message) = self.receiver.recv().await {
            self.handle(message).await;
        }
    }

    fn new(receiver: Receiver<crate::actors::Message<Self::Msg>>) -> Self {
        Self { receiver }
    }
}
