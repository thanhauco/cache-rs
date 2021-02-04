use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::fs::File;
use std::io::Write;
struct Entry {
    val: String,
    exp: Option<Instant>,
}
pub struct Store {
    data: HashMap<String, Entry>,
}
impl Store {
    pub fn new() -> Self {
        Store { data: HashMap::new() }
    }
    pub fn set(&mut self, key: String, value: String, ttl: u64) {
        let exp = if ttl > 0 { Some(Instant::now() + Duration::from_secs(ttl)) } else { None };
        self.data.insert(key, Entry { val: value, exp });
    }
    pub fn get(&mut self, key: &str) -> Option<String> {
        if let Some(e) = self.data.get(key) {
            if let Some(exp) = e.exp {
                if Instant::now() > exp {
                    self.data.remove(key);
                    return None;
                }
            }
            return Some(e.val.clone());
        }
        None
    }
    pub fn save(&self) {
        let mut f = File::create("dump.rdb").unwrap();
        for (k, v) in &self.data {
            writeln!(f, "{},{}", k, v.val).unwrap();
        }
    }
}