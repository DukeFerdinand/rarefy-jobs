mod message;
mod redis_publisher;
mod redis_subscriber;
mod crawler;
mod prisma;

#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate dotenv;

use tokio;

use message::{Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    simple_logger::SimpleLogger::new().env().init()?;
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

    // Start the async crawler thread
    info!("Spawning work loop...");
    loop {
        let msg = rx.recv();

        if let Ok(msg) = msg {
            let message_obj = Message::from_js_string(msg);

            tokio::spawn(async move {
               info!("Received message: {:?}", message_obj);
            });
        } else {
            continue
        }
    }
}
