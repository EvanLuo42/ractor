use tracing::{info, Level};
use crate::actors::{ActorHandle, Message};
use crate::actors::network::NetworkActor;

pub mod actors;
pub mod protos;
mod errors;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
    info!("Launching network actor...");
    let actor_handle = ActorHandle::new::<NetworkActor>();
    actor_handle.send(Message::new("".into())).await.unwrap();
    info!("Network actor launched!");
    loop {}
}
