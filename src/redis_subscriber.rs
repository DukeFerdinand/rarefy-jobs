extern crate serde_json;
extern crate tokio;
use futures::executor::block_on;

use crate::message::Message;
use redis::{ControlFlow, PubSubCommands};
use std::error::Error;

pub fn subscribe(channel: String, tx: crossbeam::channel::Sender<String>) -> Result<(), Box<dyn Error>> {
    let _ = tokio::spawn(async move {
        let client = redis::Client::open("redis://localhost").unwrap();
        let mut con = client.get_connection().unwrap();

        let _: () = con
            .subscribe(&[channel], |msg| {
                let received: String = msg.get_payload().unwrap();
                let message_obj = serde_json::from_str::<Message>(&received).unwrap();

                println!("{:?}", message_obj.payload);
                // block_on(tx.send(received)).unwrap();
                tx.send(received).unwrap();

                return ControlFlow::Continue;
            })
            .unwrap();
    });

    Ok(())
}
