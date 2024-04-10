use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;

use crate::actors::{Actor, Message};

pub struct SceneActor {
    receiver: Receiver<Message<String>>
}

#[async_trait]
impl Actor<String> for SceneActor {
    async fn handle(&self, message: Message<String>) {
        println!("{}", message.data)
    }

    async fn run(&mut self) {
        while let Some(message) = self.receiver.recv().await {
            self.handle(message).await;
        }
    }

    fn new(receiver: Receiver<Message<String>>) -> Self {
        Self { receiver }
    }
}
