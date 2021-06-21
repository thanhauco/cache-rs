pub struct Bloom {
    bits: Vec<bool>,
}
impl Bloom {
    pub fn check(&self, key: &str) -> bool { true }
}