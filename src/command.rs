pub enum Command {
    Set(String, String),
    Get(String),
    Unknown,
}
impl Command {
    pub fn parse(input: &str) -> Command {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        match parts.as_slice() {
            ["SET", k, v] => Command::Set(k.to_string(), v.to_string()),
            ["GET", k] => Command::Get(k.to_string()),
            _ => Command::Unknown,
        }
    }
}