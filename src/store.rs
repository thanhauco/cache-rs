use std::collections::HashMap;
pub struct Store {
    data: HashMap<String, String>,
}
impl Store {
    pub fn new() -> Self {
        Store { data: HashMap::new() }
    }
}