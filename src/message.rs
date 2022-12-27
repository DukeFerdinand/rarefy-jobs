use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum Payload {
    Crawler(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub channel: String,
    pub payload: Payload,
}

impl Message {
    pub fn new(payload: Payload) -> Message {
        Message {
            id: Message::generate_id(),
            channel: String::from("crawler"),
            payload,
        }
    }

    fn generate_id() -> String {
        return Uuid::new_v4().to_string();
    }
}
