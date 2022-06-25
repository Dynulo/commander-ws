use serde::{Serialize, Deserialize};

pub mod guild;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Root {
    Version,
    Unauthenticated(Unauthenticated),
    Authenticated(Authenticated),
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Authenticated {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    Guild(guild::Event),
}
