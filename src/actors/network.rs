use std::env;
use std::net::SocketAddr;
use async_trait::async_trait;
use sqlx::{Pool, Sqlite};
use tokio::net::TcpListener;
use tokio::sync::mpsc::Receiver;
use tracing::{error, info, trace};

use crate::actors::{Actor, ActorHandle};
use crate::actors::scene::{ScenesActor, ScenesMessage};
use crate::errors::{ErrorCode, respond_error};

#[derive(Clone, Debug)]
pub struct AppContext {
    pub(crate) db_pool: Pool<Sqlite>
}

#[derive(Debug)]
pub struct NetworkActor {
    receiver: Receiver<AppContext>,
}

#[async_trait]
impl Actor for NetworkActor {
    type Msg = AppContext;

    async fn handle(&self, message: Self::Msg) {
        let addr = env::var("HOST_ADDRESS").unwrap_or("127.0.0.1:6379".into());
        let addr: SocketAddr = addr.parse().expect("Invalid host address!");
        let listener = TcpListener::bind(addr).await.unwrap();
        info!("Created TcpListener on {}", addr);
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
            let scenes_message = ScenesMessage {
                app_context: message.clone(),
                stream: socket
            };
            scenes_handle.send(scenes_message).await.unwrap();
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
