mod store;
use store::Store;
fn main() {
    let mut s = Store::new();
    s.set("k".to_string(), "v".to_string());
    println!("{:?}", s.get("k"));
}