extern crate serde_json;
extern crate tokio;

use redis::{ControlFlow, PubSubCommands};
use std::error::Error;
use std::env;

pub fn subscribe(channel: String, tx: crossbeam::channel::Sender<String>) -> Result<(), Box<dyn Error>> {
    let redis_url = env::var("REDIS_URL")?;
    let _ = tokio::spawn(async move {
        let client = redis::Client::open(redis_url).unwrap();
        let mut con = client.get_connection().unwrap();

        let _: () = con
            .subscribe(&[channel], |msg| {
                let received: String = msg.get_payload().unwrap();
                tx.send(received).unwrap();

                return ControlFlow::Continue;
            })
            .unwrap();
    });

    Ok(())
}
