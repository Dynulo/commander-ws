use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Root {
    Version,
    Unauthenticated(Unauthenticated),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Unauthenticated {
    Login(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LoginResponse {
    Ok,
    InvalidToken,
    UnknownError,
}
