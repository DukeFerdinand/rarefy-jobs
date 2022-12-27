mod message;
mod redis_publisher;
mod redis_subscriber;
mod crawler;
mod prisma;

#[macro_use]
extern crate log;
extern crate simple_logger;

use std::thread::sleep;
use std::time::Duration;
use message::{Message};

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    info!("Running crawler system");


    // Create a cross channel message bus
    let (tx, rx) = crossbeam::channel::unbounded::<String>();

    // Connect to redis and subscribe to the channel
    if let Err(error) = redis_subscriber::subscribe(String::from("crawler"), tx) {
        error!("{:?}", error);
        panic!("{:?}", error);
    } else {
        info!("connected to queue");
    }

    info!("Waiting a few seconds for redis to subscribe...");
    sleep(Duration::from_secs(3));
    info!("ready!");

    // Start the async crawler thread
    info!("Spawning work loop...");
    loop {
        info!("Waiting for message...");
        let msg = rx.recv();

        if let Ok(msg) = msg {
            let message_obj = serde_json::from_str::<Message>(&msg).unwrap();

            tokio::spawn(async move {
               println!("Received message: {:?}", message_obj);
                // handle message
            });
        } else {
            continue
        }
    }

    // println!("Spawning message publisher thread...");
    // tokio::spawn(async move {
    //     println!("Sending a test message...");
    //
    //     loop {
    //         println!("Sending a test message...");
    //         let res = redis_publisher::publish_message(Message::new(Payload::Crawler(vec!["test".to_string()])));
    //         if let Err(error) = res {
    //             println!("Error publishing message: {:?}", error);
    //         }
    //         sleep(Duration::from_secs(3));
    //     }
    // });
}
