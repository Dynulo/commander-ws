use serde::{Deserialize, Serialize};

pub mod features;
pub mod members;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    Members(members::Event),
}
