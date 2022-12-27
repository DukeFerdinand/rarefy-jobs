use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum Payload {
    Crawler(Vec<String>),
}

type JobId = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct JavaScriptPayload {
    #[serde(rename = "jobType")]
    pub job_type: String,
    #[serde(rename = "jobData")]
    pub job_data: Vec<JobId>,
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

    // This is to massage the payload into a format that the crawler can understand
    // instead of making the JavaScript code understand the Rust format
    pub fn from_js_string(payload: String) -> Message {
        let payload: JavaScriptPayload = serde_json::from_str(&payload).unwrap();
        let payload = match payload.job_type.as_str() {
            "crawler" => Payload::Crawler(payload.job_data),
            _ => {
                error!("Unknown job type: {}", payload.job_type);
                panic!("Unknown job type");
            },
        };
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

#[cfg(test)]
mod tests {
    #[test]
    fn msg_handles_proper_json() {
        let msg = r#"{"jobType":"crawler","jobData":["1","2","3"]}"#;
        let message = super::Message::from_js_string(msg.to_string());
        assert_eq!(message.id.len(), 36);
        assert_eq!(message.channel, "crawler");
        match message.payload {
            super::Payload::Crawler(data) => {
                assert_eq!(data.len(), 3);
                assert_eq!(data[0], "1");
                assert_eq!(data[1], "2");
                assert_eq!(data[2], "3");
            },
            _ => panic!("Unexpected payload type"),
        }
    }

    #[test]
    fn msg_handles_empty_job_data() {
        let msg = r#"{"jobType":"crawler","jobData":[]}"#;
        let message = super::Message::from_js_string(msg.to_string());
        assert_eq!(message.id.len(), 36);
        assert_eq!(message.channel, "crawler");
        match message.payload {
            super::Payload::Crawler(data) => {
                assert_eq!(data.len(), 0);
            },
            _ => panic!("Unexpected payload type"),
        }
    }
}
