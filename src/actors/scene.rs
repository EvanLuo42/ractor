use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use bytes::BytesMut;
use prost::Message;
use sqlx::Database;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::Receiver;
use tokio::time::timeout;
use tracing::{debug, error, info, trace};

use crate::actors::{Actor, ActorHandle};
use crate::actors::network::AppContext;
use crate::configs::TIMEOUT;
use crate::errors::{ErrorCode, respond_error};

use crate::protos::scenes::{SelectSceneRequest, SelectSceneResponse};

#[derive(Debug)]
pub struct ScenesMessage<DB: Database> {
    pub(crate) app_context: Arc<AppContext<DB>>,
    pub(crate) stream: TcpStream
}

#[derive(Debug)]
pub enum Scene {
    SceneA
}

impl Scene {
    pub fn new_handle<DB: Database>(&self) -> ActorHandle<SceneMessage<DB>> {
        match self {
            Scene::SceneA => ActorHandle::new::<SceneAActor<DB>>()
        }
    }
}

#[derive(Debug)]
pub struct ScenesActor<DB: Database> {
    receiver: Receiver<ScenesMessage<DB>>,
    scenes: HashMap<u32, Scene>
}

#[async_trait]
impl<DB: Database> Actor for ScenesActor<DB> {
    type Msg = ScenesMessage<DB>;

    async fn handle(&self, mut message: Self::Msg) {
        info!("Scenes Actor is handling request from {:?}...", match message.stream.peer_addr() {
            Ok(addr) => addr,
            Err(e) => {
                error!("{:?}", e);
                respond_error(message.stream, ErrorCode::NetworkError).await;
                return
            }
        });
        trace!("Reading SelectSceneRequest frame from TcpStream...");
        let length = match timeout(TIMEOUT, message.stream.read_u8()).await {
            Ok(length) => length.unwrap(),
            Err(e) => {
                error!("{:?}", e);
                respond_error(message.stream, ErrorCode::DecodeProtoFailed).await;
                return
            }
        };
        let mut frame = vec![0, length];
        if let Err(e) = timeout(TIMEOUT, message.stream.read_exact(&mut frame)).await {
            error!("{:?}", e);
            respond_error(message.stream, ErrorCode::NetworkError).await;
            return
        }
        trace!("Finished reading SelectSceneRequest!");
        trace!("Decoding request into struct...");
        let request = match SelectSceneRequest::decode(&*frame) {
            Ok(request) => request,
            Err(e) => {
                error!("{:?}", e);
                return
            }
        };
        debug!("Decoded Result: {:?}", request);
        trace!("Finished decoding request into struct!");
        trace!("Finding scene with request...");
        let handle = match self.scenes.get(&request.scene_id) {
            None => {
                error!("Scene not exist!");
                respond_error(message.stream, ErrorCode::SceneNotExist).await;
                return;
            },
            Some(scene) => {
                debug!("Finished finding scene with request!");
                scene.new_handle()
            }
        };
        let scene_message = SceneMessage {
            app_context: Arc::clone(&message.app_context),
            stream: message.stream
        };
        if let Err(e) = handle.send(scene_message).await {
            error!("{:?}", e);
        }
    }

    async fn run(&mut self) {
        while let Some(message) = self.receiver.recv().await {
            self.handle(message).await;
        }
    }

    fn new(receiver: Receiver<Self::Msg>) -> Self {
        let mut scenes = HashMap::new();
        scenes.insert(1, Scene::SceneA);
        Self { receiver, scenes }
    }
}

#[derive(Debug)]
pub struct SceneMessage<DB: Database> {
    app_context: Arc<AppContext<DB>>,
    stream: TcpStream
}

#[derive(Debug)]
pub struct SceneAActor<DB: Database> {
    receiver: Receiver<SceneMessage<DB>>
}

#[async_trait]
impl<DB: Database> Actor for SceneAActor<DB> {
    type Msg = SceneMessage<DB>;

    async fn handle(&self, mut message: Self::Msg) {
        info!("Scene A Actor is handling request from {:?}...", match message.stream.peer_addr() {
            Ok(addr) => addr,
            Err(e) => {
                error!("{:?}", e);
                respond_error(message.stream, ErrorCode::NetworkError).await;
                return
            }
        });

        trace!("Encoding SelectSceneResponse to binary...");
        let response = SelectSceneResponse {
            success: true,
        };
        let length = response.encoded_len();
        let mut frame = BytesMut::with_capacity(length);
        response.encode(&mut frame).unwrap();

        trace!("Finished encoding SelectSceneResponse to binary...");

        trace!("Writing binary to frame...");
        message.stream.write_u8(length as u8).await.unwrap();
        message.stream.write_buf(&mut frame).await.unwrap();
        trace!("Finished binary to frame...");

        let mut frame = BytesMut::with_capacity(64);
        trace!("Reading client request...");
        message.stream.read_buf(&mut frame).await.unwrap();
        debug!("{:?}", frame);
    }

    async fn run(&mut self) {
        while let Some(message) = self.receiver.recv().await {
            self.handle(message).await;
        }
    }

    fn new(receiver: Receiver<Self::Msg>) -> Self {
        Self { receiver }
    }
}
