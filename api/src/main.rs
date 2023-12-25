use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Deserialize, Serialize)]
struct Post {
    userId: u32,
    id: u32,
    title: String,
    body: String,
}

async fn fetch_posts() -> Result<Vec<Post>, reqwest::Error> {
    reqwest::get("https://jsonplaceholder.typicode.com/posts")
        .await?
        .json::<Vec<Post>>()
        .await
}

#[tokio::main]
async fn main() {
    let posts_route = warp::path("posts")
        .and(warp::get())
        .then(|| async {
            let posts = fetch_posts().await.unwrap_or_else(|_| Vec::new());
            warp::reply::json(&posts)
        });

    let routes = posts_route.with(warp::cors().allow_any_origin());
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
