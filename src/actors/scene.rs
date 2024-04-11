use std::time::Duration;
use async_trait::async_trait;
use tokio::net::TcpStream;
use tokio::sync::mpsc::Receiver;
use tokio::time::sleep;

use crate::actors::{Actor, Message};

pub struct ScenesActor {
    receiver: Receiver<Message<TcpStream>>
}

#[async_trait]
impl Actor for ScenesActor {
    type Msg = TcpStream;

    async fn handle(&self, message: Message<Self::Msg>) {
        println!("A");
        sleep(Duration::new(5, 0)).await;
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
