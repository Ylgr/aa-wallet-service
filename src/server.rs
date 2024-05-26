use std::net::SocketAddr;
use axum::{Json, Router};
use axum::routing::get;
use anyhow::Result;

pub async fn start_server() -> Result<()> {
    let app = Router::new()
        .route("/", get(get_items));
    // .route("/", post(create_item))
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
