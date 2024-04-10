use std::sync::Arc;
use tokio::sync::Mutex;
use crate::actors::{ActorHandle, Message};
use crate::actors::network::NetworkActor;
use crate::actors::scene::SceneActor;

pub mod actors;

#[tokio::main]
async fn main() {
    let actor_handle = ActorHandle::new::<NetworkActor>();
    let scene1 = ActorHandle::new::<SceneActor>();
    actor_handle.send(Message::new(Arc::new(Mutex::new(vec![scene1])))).await.unwrap();
    loop {}
}