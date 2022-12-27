mod message;
mod redis_publisher;
mod redis_subscriber;

use std::thread::sleep;
use std::time::Duration;
use message::{Message, Payload};

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running crawler system");
    if let Err(error) = redis_subscriber::subscribe(String::from("crawler")) {
        println!("{:?}", error);
        panic!("{:?}", error);
    } else {
        println!("connected to queue");
    }

    println!("Waiting a few seconds for redis to subscribe...");
    sleep(Duration::from_secs(3));
    println!("ready!");
    //
    // println!("Sending a test message...");
    // redis_publisher::publish_message(Message::new(Payload::Crawler(vec!["test".to_string()])))?;
    //
    // println!("Sending a test message...");
    // redis_publisher::publish_message(Message::new(Payload::Crawler(vec!["test".to_string()])))?;
    //
    // println!("Sending a test message...");
    // redis_publisher::publish_message(Message::new(Payload::Crawler(vec!["test".to_string()])))?;

    Ok(())
}
