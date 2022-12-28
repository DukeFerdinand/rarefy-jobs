mod message;
mod redis_publisher;
mod redis_subscriber;
mod prisma;
mod crawler;

#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate dotenv;

use tokio;

use message::{Message, Payload};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    simple_logger::init_with_env()?;
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

        // Check if we have a message
        if let Ok(msg) = msg {
            // Convert it into a shape we expect
            let message_obj = Message::from_js_string(msg);

            // Check what kind of job we're dealing with
            match message_obj.payload {
                // TODO: Move this match arm block into another function for maintainability
                Payload::Crawler(ids) => {
                    // See if there are any pre-selected searches to scrape
                    if let Some(selected) = ids {
                        info!("Received crawler job with {:?} ids", selected);
                    } else {
                        info!("Received crawler job with no pre-selections, preparing to scrape all!");
                        crawler::find_jobs::find_jobs().await.unwrap();
                    }
                }
            }


            // tokio::spawn(async move {
            //
            // });
        } else {
            continue
        }
    }
}
