use tokio::fs::File;
use tokio::io::AsyncWriteExt;
pub struct Aof {
    file: File,
}
impl Aof {
    pub async fn new() -> Self {
        let file = File::create("appendonly.aof").await.unwrap();
        Aof { file }
    }
    pub async fn write(&mut self, cmd: &str) {
        self.file.write_all(cmd.as_bytes()).await.unwrap();
    }
}