use simple_redis::{network, storage::memory::InMemStore};
use tokio::net::TcpListener;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:8088";
    info!("listen on: {}", addr);
    let listener = TcpListener::bind(addr).await?;

    let store = InMemStore::new();
    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("accept connection from: {}", raddr);

        let inner_store = store.clone();

        tokio::spawn(async move {
            match network::stream_handler(stream, inner_store).await {
                Ok(_) => {
                    info!("connection from {} exit", raddr);
                }
                Err(e) => {
                    warn!("handle error for {}:{}", raddr, e);
                }
            }
        });
    }
}
