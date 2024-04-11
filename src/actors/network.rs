use async_trait::async_trait;
use tokio::net::TcpListener;
use tokio::sync::mpsc::Receiver;

use crate::actors::{Actor, ActorHandle, Message};
use crate::actors::scene::ScenesActor;

pub struct NetworkActor {
    receiver: Receiver<Message<String>>,
}

#[async_trait]
impl Actor for NetworkActor {
    type Msg = String;

    async fn handle(&self, _: Message<Self::Msg>) {
        let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
        while let Ok((socket, _)) = listener.accept().await {
            let scenes_handle = ActorHandle::new::<ScenesActor>();
            scenes_handle.send(Message::new(socket)).await.unwrap();
        }
    }

    async fn run(&mut self) {
        while let Some(message) = self.receiver.recv().await {
            self.handle(message).await;
        }
    }

    fn new(receiver: Receiver<Message<Self::Msg>>) -> Self {
        Self { receiver }
    }
}
