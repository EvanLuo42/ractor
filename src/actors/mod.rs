pub mod network;
pub mod scene;

use std::ops::Deref;
use async_trait::async_trait;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::mpsc::error::SendError;

#[async_trait]
pub trait Actor: Send {
    type Msg;

    async fn handle(&self, message: Message<Self::Msg>);
    async fn run(&mut self);
    fn new(receiver: Receiver<Message<Self::Msg>>) -> Self;
}

#[derive(Clone)]
pub struct Message<T> {
    pub data: T
}

impl<T> Message<T> {
    pub fn new(data: T) -> Message<T> {
        Self { data }
    }
}

impl<T: Clone> Deref for Message<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(Clone)]
pub struct ActorHandle<T> {
    sender: Sender<Message<T>>
}

impl<T> ActorHandle<T> {
    pub fn new<A: Actor<Msg = T> + 'static>() -> ActorHandle<T> {
        let (sender, receiver) = mpsc::channel(8);
        let mut actor = A::new(receiver);
        tokio::spawn(async move { actor.run().await });
        Self { sender }
    }

    pub fn from_sender(sender: Sender<Message<T>>) -> ActorHandle<T> {
        Self { sender }
    }

    pub async fn send(&self, message: Message<T>) -> Result<(), SendError<Message<T>>> {
        self.sender.send(message).await
    }
}