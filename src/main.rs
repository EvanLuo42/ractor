use dotenv::dotenv;
use sqlx::SqlitePool;
use tracing::{info, Level};
use crate::actors::ActorHandle;
use crate::actors::network::{AppContext, NetworkActor};

pub mod actors;
pub mod protos;
pub mod errors;
mod configs;
mod models;
mod query;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
    info!("Launching network actor...");
    let network_handle = ActorHandle::new::<NetworkActor>();
    let app_context = AppContext {
        db_pool: SqlitePool::connect("sqlite:db.sqlite3").await.unwrap()
    };
    network_handle.send(app_context).await.unwrap();
    info!("Network actor launched!");
    loop {}
}
