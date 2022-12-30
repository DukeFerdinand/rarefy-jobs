extern crate redis;

use std::env;
use std::error::Error;
use std::fmt::{Display, Formatter, Pointer};
use redis::{Client as RedisClient, Msg, PubSubCommands};
use redis::ControlFlow::Continue;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{info, error, warn};

pub struct PubSub {
    source: RedisClient,
    topic: &'static str
}

#[derive(Debug)]
pub enum JobType {
    Crawler,
    Other(String)
}

impl Serialize for JobType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(match *self {
            JobType::Crawler => "foo",
            JobType::Other(ref other) => other,
        })
    }
}

impl<'de> Deserialize<'de> for JobType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "crawler" => JobType::Crawler,
            _ => JobType::Other(s),
        })
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct JobConfig {
    pub job_type: JobType
}

impl PubSub {
    pub fn new(topic: &'static str) -> PubSub {
        let source = RedisClient::open(env::var("REDIS_URL").unwrap()).unwrap();

        PubSub {
            topic,
            source
        }
    }

    pub fn payload_to_job(payload: String) -> Result<JobConfig, serde_json::Error> {
        serde_json::from_str(&payload)
    }

    /// # PubSub.subscribe
    /// This subscribe function will create a blocking loop and run the given function
    pub fn subscribe<F>(&self, mut func: F) -> Result<(), Box<dyn Error>>
    where
        F: FnMut(JobConfig) -> Result<(), Box<dyn Error>>
    {
        let mut con = self.source.get_connection().unwrap();
        con.subscribe(self.topic, |msg| {
            let payload = msg.get_payload::<String>().unwrap();

            // Unmarshall the JSON and pass it to the handler
            if let Ok(msg) = PubSub::payload_to_job(payload) {
                match func(msg) {
                    // Surface any errors from the handler function
                    Err(e) => error!("{}", e),
                    _ => Continue
                }
            } else {
                // Maybe consider a better error message here? Not super important
                warn!("Found malformed job config, ignoring");

                Continue
            }
        })?
    }
}