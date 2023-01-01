extern crate tokio;

use crate::collector::Collector;
use crate::pubsub::{JobEvent, JobType, PubSub};
use crate::{error, info};
use crossbeam::channel::{Receiver, Sender};

/// The entrypoint for the Rarefy scraper
pub struct RareFinder {
    transmitter: Sender<JobEvent>,
    receiver: Receiver<JobEvent>,
    pubsub: PubSub,
}

impl RareFinder {
    pub fn new() -> RareFinder {
        let (tx, rx) = crossbeam::channel::unbounded();

        RareFinder {
            transmitter: tx,
            receiver: rx,
            pubsub: PubSub::new("crawler"),
        }
    }

    /// # RareFinder.run
    /// This runs the whole scraper stack, including the redis sub client
    pub fn run(&mut self) {
        tokio_scoped::scope(|thread_scope| {
            // this must be in a new thread or other block-remover
            // as the redis subscriber function spawns an infinite blocking loop
            // NOTE: at time of writing this is not async, but it can be with tokio
            thread_scope.spawn(async {
                // Spawn the blocking subscriber
                let res = self.pubsub.subscribe(|job| {
                    let res = self.transmitter.send(job);

                    if let Err(e) = res {
                        error!("Got error sending message: {}", e)
                    }

                    Ok(())
                });

                if let Err(e) = res {
                    error!("Got error from subscriber: {}", e)
                }
            });

            info!("Starting job loop");
            loop {
                // Block until a message is received
                let msg = self.receiver.recv();

                // Handle message if okay
                if let Ok(job) = msg {
                    // Then spawn a thread to handle this message
                    thread_scope.spawn(async move {
                        let collector = Collector::new();
                        // Check the job type and handle
                        match &job.job_type {
                            // TODO: consider moving this into a separate function - self contained?
                            JobType::Crawler => {
                                info!("Running crawler job");
                                if let Err(e) = collector.collect_from_all().await {
                                    error!("Got error running collect_from_all: {}", e)
                                } else {
                                    info!("Success from crawler job")
                                }
                            }
                            _ => unimplemented!(),
                        }
                    });
                } else {
                    error!("Error when receiving message!");
                }
            }
        });
    }
}
