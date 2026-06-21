/// HTTP facade — stub surface until a backend (e.g. `reqwest` over [`super::super::rt::Rt`]) is wired in.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Net;

/// A request to send.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Request {

    pub method: String,
    pub url: String,
    pub body: Option<String>,
}

/// A received response.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Response {

    pub status: u16,
    pub body: String,
}
