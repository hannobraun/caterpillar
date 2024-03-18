#[tokio::main]
async fn main() {
    use warp::Filter;
    warp::serve(warp::any().map(|| "Hello, world!"))
        .run(([127, 0, 0, 1], 8080))
        .await;
}
