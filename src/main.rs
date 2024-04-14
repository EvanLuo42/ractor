use tracing::{info, Level};
use crate::actors::{ActorHandle, Message};
use crate::actors::network::NetworkActor;

pub mod actors;
pub mod protos;
pub mod errors;
mod configs;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
    info!("Launching network actor...");
    let network_handle = ActorHandle::new::<NetworkActor>();
    network_handle.send(Message::new("".into())).await.unwrap();
    info!("Network actor launched!");
    loop {}
}
