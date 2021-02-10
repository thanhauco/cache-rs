use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::fs::File;
use std::io::Write;
struct Entry {
    val: String,
    exp: Option<Instant>,
    last_access: Instant,
}
pub struct Store {
    data: HashMap<String, Entry>,
    capacity: usize,
}
impl Store {
    pub fn new() -> Self {
        Store { data: HashMap::new(), capacity: 100 }
    }
    pub fn set(&mut self, key: String, value: String, ttl: u64) {
        if self.data.len() >= self.capacity {
            // Simple random eviction for now
            let k = self.data.keys().next().unwrap().clone();
            self.data.remove(&k);
        }
        let exp = if ttl > 0 { Some(Instant::now() + Duration::from_secs(ttl)) } else { None };
        self.data.insert(key, Entry { val: value, exp, last_access: Instant::now() });
    }
    pub fn get(&mut self, key: &str) -> Option<String> {
        if let Some(e) = self.data.get_mut(key) {
            if let Some(exp) = e.exp {
                if Instant::now() > exp {
                    self.data.remove(key);
                    return None;
                }
            }
            e.last_access = Instant::now();
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