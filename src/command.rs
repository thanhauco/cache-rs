pub enum Command {
    Set(String, String),
    Get(String),
    Unknown,
}