pub enum Command {
    Set(String, String, u64),
    Get(String),
    Save,
    Unknown,
}
impl Command {
    pub fn parse(input: &str) -> Command {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        match parts.as_slice() {
            ["SET", k, v, ttl] => Command::Set(k.to_string(), v.to_string(), ttl.parse().unwrap_or(0)),
            ["SET", k, v] => Command::Set(k.to_string(), v.to_string(), 0),
            ["GET", k] => Command::Get(k.to_string()),
            ["SAVE"] => Command::Save,
            _ => Command::Unknown,
        }
    }
}