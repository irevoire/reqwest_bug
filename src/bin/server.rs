use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "hello reqwest" }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:7777".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
