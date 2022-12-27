mod message;
mod redis_publisher;
mod redis_subscriber;
mod crawler;

use std::thread::sleep;
use std::time::Duration;
use message::{Message, Payload};

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running crawler system");

    // Create a cross channel message bus
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(32);

    // Connect to redis and subscribe to the channel
    if let Err(error) = redis_subscriber::subscribe(String::from("crawler"), tx) {
        println!("{:?}", error);
        panic!("{:?}", error);
    } else {
        println!("connected to queue");
    }

    println!("Waiting a few seconds for redis to subscribe...");
    sleep(Duration::from_secs(3));
    println!("ready!");

    // Start the async crawler thread
    println!("Spawning crawler thread");
    tokio::spawn(async move {
        loop {
            let msg = rx.recv().await;

            match msg {
                Some(msg) => {
                    println!("Received message: {}", msg);
                },
                None => continue,
            }
        }
    });

    //
    println!("Sending a test message...");
    redis_publisher::publish_message(Message::new(Payload::Crawler(vec!["test".to_string()])))?;

    // println!("Sending a test message...");
    // redis_publisher::publish_message(Message::new(Payload::Crawler(vec!["test".to_string()])))?;
    //
    // println!("Sending a test message...");
    // redis_publisher::publish_message(Message::new(Payload::Crawler(vec!["test".to_string()])))?;

    Ok(())
}
