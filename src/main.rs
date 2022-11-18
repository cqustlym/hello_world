use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(test));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap()).serve(app.into_make_service())
        .await.unwrap();
}

async fn test() -> String {
    String::from("lym")
}