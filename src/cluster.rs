pub struct Cluster {
    nodes: Vec<String>,
}
impl Cluster {
    pub fn get_node(&self, key: &str) -> String {
        // CRC16 mock
        self.nodes[0].clone()
    }
}