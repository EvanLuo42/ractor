use std::collections::HashMap;

use async_trait::async_trait;
use bytes::BytesMut;
use prost::Message;
use sqlx::{query, query_as, Sqlite};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::Receiver;
use tokio::time::timeout;
use tracing::{debug, error, info, trace};

use crate::actors::{Actor, ActorHandle};
use crate::actors::network::AppContext;
use crate::configs::TIMEOUT;
use crate::errors::{ErrorCode, respond_error};
use crate::models::scenes::Test;

use crate::protos::scenes::{SelectSceneRequest, SelectSceneResponse};
use crate::query::scenes::get_all_tests;

#[derive(Debug)]
pub struct ScenesMessage {
    pub(crate) app_context: AppContext,
    pub(crate) stream: TcpStream
}

#[derive(Debug)]
pub enum Scene {
    SceneA
}

impl Scene {
    pub fn new_handle(&self) -> ActorHandle<SceneMessage> {
        match self {
            Scene::SceneA => ActorHandle::new::<SceneAActor>()
        }
    }
}

#[derive(Debug)]
pub struct ScenesActor {
    receiver: Receiver<ScenesMessage>,
    scenes: HashMap<u32, Scene>
}

#[async_trait]
impl Actor for ScenesActor {
    type Msg = ScenesMessage;

    async fn handle(&self, mut message: Self::Msg) {
        info!("Scenes Actor is handling request from {:?}...", match message.stream.peer_addr() {
            Ok(addr) => addr,
            Err(e) => {
                error!("{:?}", e);
                respond_error(message.stream, ErrorCode::NetworkError).await;
                return
            }
        });
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
        let request = match SelectSceneRequest::decode(&*frame) {
            Ok(request) => request,
            Err(e) => {
                error!("{:?}", e);
                return
            }
        };
        debug!("Decoded Result: {:?}", request);
        let handle = match self.scenes.get(&request.scene_id) {
            None => {
                error!("Scene not exist!");
                respond_error(message.stream, ErrorCode::SceneNotExist).await;
                return;
            },
            Some(scene) => {
                scene.new_handle()
            }
        };
        let scene_message = SceneMessage {
            app_context: message.app_context,
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
pub struct SceneMessage {
    app_context: AppContext,
    stream: TcpStream
}

#[derive(Debug)]
pub struct SceneAActor {
    receiver: Receiver<SceneMessage>
}

#[async_trait]
impl Actor for SceneAActor {
    type Msg = SceneMessage;

    async fn handle(&self, mut message: Self::Msg) {
        info!("Scene A Actor is handling request from {:?}...", match message.stream.peer_addr() {
            Ok(addr) => addr,
            Err(e) => {
                error!("{:?}", e);
                respond_error(message.stream, ErrorCode::NetworkError).await;
                return
            }
        });

        let response = SelectSceneResponse {
            success: true,
        };
        let length = response.encoded_len();
        let mut frame = BytesMut::with_capacity(length);
        response.encode(&mut frame).unwrap();
        debug!("Encoded Response: {:?}", response);

        message.stream.write_u8(length as u8).await.unwrap();
        message.stream.write_buf(&mut frame).await.unwrap();
        let tests = get_all_tests(&message.app_context.db_pool).await.unwrap();
        debug!("{:?}", tests);

        let mut frame = BytesMut::with_capacity(64);
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
