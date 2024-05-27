mod server;
mod alchemy_webhook_dto;

#[tokio::main]
async fn main() {
    server::start_server().await.unwrap();
}

