use warp::Filter;

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "DELETE", "PUT", "OPTIONS"])
        .allow_headers(vec!["Content-Type", "User-Agent", "Authorization"]);

    let hello_world = warp::path!("hello" / "world")
        .map(|| warp::reply::html("Hello, Rust World!"))
        .with(cors);  // CORS設定を適用

    warp::serve(hello_world)
        .run(([127, 0, 0, 1], 8080))
        .await;
}