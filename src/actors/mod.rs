pub mod network;
pub mod scene;

use std::fmt::Debug;
use async_trait::async_trait;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::mpsc::error::SendError;
use tracing::debug;

#[async_trait]
pub trait Actor: Send + Debug {
    type Msg;

    async fn handle(&self, message: Self::Msg);
    async fn run(&mut self);
    fn new(receiver: Receiver<Self::Msg>) -> Self;
}
#[derive(Clone)]
pub struct ActorHandle<T> {
    sender: Sender<T>
}

impl<T> ActorHandle<T> {
    pub fn new<A: Actor<Msg = T> + 'static>() -> ActorHandle<T> {
        let (sender, receiver) = mpsc::channel(8);
        let mut actor = A::new(receiver);
        tokio::spawn(async move {
            debug!("Running actor: {:?}", actor);
            actor.run().await;
        });
        Self { sender }
    }

    pub fn from_sender(sender: Sender<T>) -> ActorHandle<T> {
        Self { sender }
    }

    pub async fn send(&self, message: T) -> Result<(), SendError<T>> {
        self.sender.send(message).await
    }
}