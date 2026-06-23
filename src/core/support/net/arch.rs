use std::sync::OnceLock;

pub static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

#[derive(Debug, Clone, Default)]
pub struct Request {
    pub method: String,
    pub url: String,
    pub headers: Vec<( String, String )>,
    pub query: Vec<( String, String )>,
    pub body: Option<String>,
    pub basic_auth: Option<( String, String )>,
    pub timeout: u64,
    pub retries: u32,
}

#[derive(Debug, Clone, Default)]
pub struct Response {
    pub status: u16,
    pub url: String,
    pub headers: Vec<( String, String )>,
    pub body: String,
}
