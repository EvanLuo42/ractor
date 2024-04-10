use std::sync::Arc;
use tokio::sync::Mutex;
use async_trait::async_trait;
use tokio::net::TcpListener;
use tokio::sync::mpsc::Receiver;

use crate::actors::{Actor, ActorHandle, Message};

type Scenes = Arc<Mutex<Vec<ActorHandle<String>>>>;

pub struct NetworkActor {
    receiver: Receiver<Message<Scenes>>,
}

#[async_trait]
impl Actor<Scenes> for NetworkActor {
    async fn handle(&self, message: Message<Scenes>) {
        let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
        while let Ok((socket, _)) = listener.accept().await {
            let message_arc = Arc::clone(&message.data);
            tokio::spawn(async move {
                message_arc
                    .lock()
                    .await
                    .first()
                    .unwrap()
                    .send(Message::new(socket.local_addr().unwrap().to_string()))
                    .await
                    .unwrap();
            });
        }
    }

    async fn run(&mut self) {
        while let Some(message) = self.receiver.recv().await {
            self.handle(message).await;
        }
    }

    fn new(receiver: Receiver<Message<Scenes>>) -> Self {
        Self { receiver }
    }
}
