use async_trait::async_trait;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc::Receiver;

use crate::actors::{Actor, Message};

pub struct ScenesActor {
    receiver: Receiver<Message<TcpStream>>
}

#[async_trait]
impl Actor for ScenesActor {
    type Msg = TcpStream;

    async fn handle(&self, mut message: Message<Self::Msg>) {
        let mut packet = BufReader::new(&mut message.data);
        let mut frame = vec![];
        packet.read_until('\n' as u8, &mut frame).await.unwrap();
        let raw = String::from_utf8(frame).unwrap();
        println!("{:?}", raw.split_ascii_whitespace());
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
