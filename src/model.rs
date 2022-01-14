use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Item {
    pub key: String,
    pub value: String,
}