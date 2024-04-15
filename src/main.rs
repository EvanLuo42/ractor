use std::sync::Arc;
use sqlx::{Sqlite, SqlitePool};
use tracing::{info, Level};
use crate::actors::ActorHandle;
use crate::actors::network::{AppContext, NetworkActor};

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
    let network_handle = ActorHandle::new::<NetworkActor<Sqlite>>();
    let app_context = AppContext {
        db_pool: SqlitePool::connect("sqlite").await.unwrap()
    };
    network_handle.send(Arc::new(app_context)).await.unwrap();
    info!("Network actor launched!");
    loop {}
}
