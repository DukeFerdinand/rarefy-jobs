extern crate redis;
extern crate serde_json;


use crate::message::Message;
use redis::Commands;
use std::error::Error;
use std::env;

pub fn publish_message(channel: &str, message: &str) -> Result<(), Box<dyn Error>> {
    let redis_url = env::var("REDIS_URL")?;
    let client = redis::Client::open(redis_url)?;
    let mut con = client.get_connection()?;

    con.publish(channel, message)?;

    Ok(())
}
