use std::net::SocketAddr;
use axum::{Json, Router};
use axum::routing::{get, post};
use anyhow::Result;
use crate::alchemy_webhook_dto;

pub async fn start_server() -> Result<()> {
    let app = Router::new()
        .route("/", get(get_items))
        .route("/webhook/marketplace/bid", post(bid_on_marketplace_auction));
    // .route("/:id", put(update_item))
    // .route("/:id", delete(delete_item));

    println!("Hello, world!");
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn get_items() -> Json<Vec<String>> {
    Json(vec![
        "Item 1".to_string(),
        "Item 2".to_string(),
        "Item 3".to_string(),
    ])
}

async fn bid_on_marketplace_auction(
    Json(dto): Json<alchemy_webhook_dto::AlchemyWebhookDto>
) -> Json<alchemy_webhook_dto::AlchemyWebhookDto> {
    println!("{:?}", dto);
    Json(dto)
}
