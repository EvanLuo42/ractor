use crate::actors::ActorHandle;
use crate::actors::network::NetworkActor;

pub mod actors;

#[tokio::main]
async fn main() {
    let _ = ActorHandle::new::<NetworkActor>();

    loop {}
}