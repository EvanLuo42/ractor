use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;
use crate::actor::{Actor, ActorHandle, Message};

pub mod actor;

#[tokio::main]
async fn main() {
    let handle1: ActorHandle<ActorMessageTypes> = ActorHandle::new::<Actor1>();
    handle1.sender.send(Message::new(ActorMessageTypes::DoA)).await.unwrap();
    handle1.sender.send(Message::new(ActorMessageTypes::DoA)).await.unwrap();
    handle1.sender.send(Message::new(ActorMessageTypes::DoA)).await.unwrap();
    handle1.sender.send(Message::new(ActorMessageTypes::DoA)).await.unwrap();
    handle1.sender.send(Message::new(ActorMessageTypes::DoB)).await.unwrap();
    loop {

    }
}

struct Actor1 {
    receiver: Receiver<Message<ActorMessageTypes>>
}

#[async_trait]
impl Actor<ActorMessageTypes> for Actor1 {
    async fn handle(&self, message: Message<ActorMessageTypes>) {
        match message.data {
            ActorMessageTypes::DoA => println!("A"),
            ActorMessageTypes::DoB => println!("B"),
        }
    }

    async fn run(&mut self) {
        while let Some(message) = self.receiver.recv().await {
            self.handle(message).await;
        }
    }

    fn new(receiver: Receiver<Message<ActorMessageTypes>>) -> Self {
        Self { receiver }
    }
}

enum ActorMessageTypes {
    DoA, DoB
}