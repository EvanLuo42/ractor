use std::env;
use std::sync::Arc;
use dotenv::dotenv;
use sqlx::SqlitePool;
use tracing::info;
use tracing_subscriber::EnvFilter;
use crate::actors::ActorHandle;
use crate::actors::network::{NetworkActor};
use crate::actors::AppContext;

pub mod actors;
pub mod protos;
pub mod errors;
pub mod configs;
pub mod models;
pub mod query;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Launching network actor...");
    let network_handle = ActorHandle::new::<NetworkActor>();
    let app_context = AppContext {
        db_pool: SqlitePool::connect(env::var("DATABASE_URL")
            .unwrap_or("sqlite:db.sqlite3".into()).as_str()).await.unwrap()
    };
    network_handle.send(Arc::new(app_context)).await.unwrap();
    info!("Network actor launched!");
    loop {}
}
