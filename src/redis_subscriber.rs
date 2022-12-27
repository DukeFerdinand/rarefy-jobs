extern crate serde_json;
extern crate tokio;

use crate::message::Message;
use redis::{ControlFlow, PubSubCommands};
use std::error::Error;

pub fn subscribe(channel: String) -> Result<(), Box<dyn Error>> {
    let _ = tokio::spawn(async move {
        let client = redis::Client::open("redis://localhost").unwrap();
        let mut con = client.get_connection().unwrap();

        let _: () = con
            .subscribe(&[channel], |msg| {
                let received: String = msg.get_payload().unwrap();
                let message_obj = serde_json::from_str::<Message>(&received).unwrap();

                println!("{:?}", message_obj.payload);

                return ControlFlow::Continue;
            })
            .unwrap();
    });

    Ok(())
}
