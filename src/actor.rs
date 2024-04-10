use async_trait::async_trait;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

#[async_trait]
pub trait Actor<T>: Send {
    async fn handle(&self, message: Message<T>);
    async fn run(&mut self);
    fn new(receiver: Receiver<Message<T>>) -> Self;
}

pub struct Message<T> {
    pub data: T
}

impl<T> Message<T> {
    pub fn new(data: T) -> Message<T> {
        Self { data }
    }
}

#[derive(Clone)]
pub struct ActorHandle<T> {
    pub sender: Sender<Message<T>>
}

impl<T> ActorHandle<T> {
    pub fn new<A: Actor<T> + 'static>() -> ActorHandle<T> {
        let (sender, receiver) = mpsc::channel(1);
        let mut actor = A::new(receiver);
        tokio::spawn(async move { actor.run().await });
        Self { sender }
    }
}