use tokio::net::TcpListener;
pub async fn run() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
}