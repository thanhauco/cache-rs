use tokio::signal;
async fn main() {
    signal::ctrl_c().await.unwrap();
    println!("Shutdown");
}