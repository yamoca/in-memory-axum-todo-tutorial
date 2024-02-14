use axum::{ http::{header::CONTENT_TYPE, HeaderValue, Method}, routing, Json, Router };
use tokio::net::TcpListener;
use tower_http;


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", routing::get(handler))
        .layer(
            tower_http::cors::CorsLayer::new() // make sure below ip matches output of yarn dev (make sure frontend is running on correct port/ip)
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_headers([CONTENT_TYPE])
                .allow_methods([Method::GET]),
        );

    let listener = TcpListener::bind("127.0.0.1:3030").await.unwrap();

    println!("server started, listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

#[derive(serde::Serialize)]
struct Message {
    message: String,
}

async fn handler() -> Json<Message> {
    Json(Message {
        message: format!("hello world"),
    })
}