use async_trait::async_trait;
use log::info;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio::sync::mpsc::Receiver;
use crate::actors::{Actor, Message};

pub struct NetworkActor;

#[async_trait]
impl Actor<u8> for NetworkActor {
    async fn handle(&self, _: Message<u8>) {
        unreachable!()
    }

    async fn run(&mut self) {
        info!("Launching TCP listener on port 6379...");
        let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
        info!("Listening for TCP request...");
        while let Ok((mut socket, _)) = listener.accept().await {
            info!("Received 1 TCP request!");
            let mut buf = [];
            println!("{}", socket.read(&mut buf).await.unwrap());
        }
    }

    fn new(_: Receiver<Message<u8>>) -> Self {
        Self { }
    }
}
