pub struct Replica {
    master_host: String,
}
impl Replica {
    pub fn sync(&self) {
        // Connect to master and sync
        println!("Syncing with master...");
    }
}