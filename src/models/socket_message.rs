use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SocketMessage {
    pub title: String,
    pub status: String,
}
