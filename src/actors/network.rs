use async_trait::async_trait;
use sqlx::{Database, Pool};
use tokio::net::TcpListener;
use tokio::sync::mpsc::Receiver;
use tracing::{error, info, trace};

use crate::actors::{Actor, ActorHandle};
use crate::actors::scene::ScenesActor;
use crate::errors::{ErrorCode, respond_error};

pub struct AppContext<DB: Database> {
    pub(crate) db_pool: Pool<DB>
}

#[derive(Debug)]
pub struct NetworkActor<DB: Database> {
    receiver: Receiver<AppContext<DB>>,
}

#[async_trait]
impl<DB: Database> Actor for NetworkActor<DB> {
    type Msg = AppContext<DB>;

    async fn handle(&self, message: Self::Msg) {
        trace!("Creating TcpListener...");
        let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
        trace!("TcpListener created!");
        while let Ok((socket, _)) = listener.accept().await {
            info!("Received a request from {:?}", match socket.peer_addr() {
                Ok(addr) => addr,
                Err(e) => {
                    error!("{:?}", e);
                    respond_error(socket, ErrorCode::NetworkError).await;
                    return
                }
            });
            let scenes_handle = ActorHandle::new::<ScenesActor>();
            scenes_handle.send(socket).await.unwrap();
        }
    }

    async fn run(&mut self) {
        while let Some(message) = self.receiver.recv().await {
            self.handle(message).await;
        }
    }

    fn new(receiver: Receiver<Self::Msg>) -> Self {
        Self { receiver }
    }
}
