use crate::actors::{ActorHandle, Message};
use crate::actors::network::NetworkActor;

pub mod actors;
mod protos;

#[tokio::main]
async fn main() {
    let actor_handle = ActorHandle::new::<NetworkActor>();
    actor_handle.send(Message::new("".into())).await.unwrap();
    loop {}
}