use std::collections::HashMap;

use async_trait::async_trait;
use bytes::BytesMut;
use prost::Message;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;

use crate::actors::{Actor, ActorHandle};
use crate::protos::scenes::SelectScene;

pub enum Scenes {
    SceneA
}

impl Scenes {
    pub fn new_handle(&self) -> ActorHandle<TcpStream> {
        match self {
            Scenes::SceneA => {
                let (sender, receiver) = mpsc::channel(8);
                let mut actor = SceneAActor::new(receiver);
                tokio::spawn(async move { actor.run().await });
                ActorHandle::from_sender(sender)
            }
        }
    }
}

pub struct ScenesActor {
    receiver: Receiver<crate::actors::Message<TcpStream>>,
    scenes: HashMap<u32, Scenes>
}

#[async_trait]
impl Actor for ScenesActor {
    type Msg = TcpStream;

    async fn handle(&self, mut message: crate::actors::Message<Self::Msg>) {
        let mut packet = BufReader::new(&mut message.data);
        let mut frame = BytesMut::with_capacity(64);
        packet.read_buf(&mut frame).await.unwrap();
        // TODO: Error handling
        let data = SelectScene::decode(frame).unwrap();
        let handle = self.scenes.get(&data.scene_id).unwrap().new_handle();
        handle.send(message).await.unwrap()
    }

    async fn run(&mut self) {
        while let Some(message) = self.receiver.recv().await {
            self.handle(message).await;
        }
    }

    fn new(receiver: Receiver<crate::actors::Message<Self::Msg>>) -> Self {
        let mut scenes = HashMap::new();
        scenes.insert(1, Scenes::SceneA);
        Self { receiver, scenes }
    }
}

pub struct SceneAActor {
    receiver: Receiver<crate::actors::Message<TcpStream>>
}

#[async_trait]
impl Actor for SceneAActor {
    type Msg = TcpStream;

    async fn handle(&self, mut message: crate::actors::Message<Self::Msg>) {
        let mut packet = BufReader::new(&mut message.data);
        let mut frame = BytesMut::with_capacity(64);
        packet.read_buf(&mut frame).await.unwrap();
        println!("{:?}", frame);
        loop {

        }
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
