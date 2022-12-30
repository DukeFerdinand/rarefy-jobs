use std::thread;
use crossbeam::channel::{Sender, Receiver};

use crate::{error, info};
use crate::pubsub::{JobConfig, PubSub};

pub struct Scraper {
    transmitter: Sender<JobConfig>,
    receiver: Receiver<JobConfig>,
    pubsub: PubSub
}

impl Scraper {
    pub fn new() -> Scraper {
        let (tx, rx) = crossbeam::channel::unbounded();

        Scraper {
            transmitter: tx,
            receiver: rx,
            pubsub: PubSub::new("crawler")
        }
    }

    /// # Scraper.run
    /// This runs the whole scraper stack, including the redis sub client
    pub fn run(&mut self) {
        thread::scope(|thread_scope| {
            let res = self.pubsub.subscribe(|job| {
                thread_scope.spawn(|| {
                    info!("Handling message! {:?}", job.job_type);
                    if let Err(e) = self.transmitter.send(job) {
                        error!("Got error sending message: {}", e)
                    }
                });
                Ok(())
            });

            if let Err(e) = res {
                error!("Got error from subscriber: {}", e)
            }

            loop {
                let msg = self.receiver.recv();
                if let Ok(_) = msg {
                    info!("Handling job");
                } else {
                    error!("Error when receiving message!");
                }
            }


        });
    }
}
