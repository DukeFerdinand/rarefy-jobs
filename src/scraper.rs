use std::thread;
use crossbeam::channel::{Sender, Receiver};

use crate::{error, info};
use crate::collector::Collector;
use crate::pubsub::{JobConfig, JobType, PubSub};

/// The entrypoint for the Rarefy scraper
pub struct Scraper {
    transmitter: Sender<JobConfig>,
    receiver: Receiver<JobConfig>,
    pubsub: PubSub,
    collector: Collector
}

impl Scraper {
    pub fn new() -> Scraper {
        let (tx, rx) = crossbeam::channel::unbounded();

        Scraper {
            transmitter: tx,
            receiver: rx,
            pubsub: PubSub::new("crawler"),
            collector: Collector::new()
        }
    }

    /// # Scraper.run
    /// This runs the whole scraper stack, including the redis sub client
    pub fn run(&mut self) {
        thread::scope(|thread_scope| {
            // this must be in a new thread or other block-remover
            // as the redis subscriber function spawns an infinite blocking loop
            thread_scope.spawn(|| {
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
                let msg = self.receiver.recv();
                if let Ok(job) = msg {
                    thread_scope.spawn(move || {
                        let collector = Collector::new();
                        match &job.job_type {
                            JobType::Crawler => {
                                info!("Running crawler job");
                                collector.collect_from_all()
                            },
                            _ => unimplemented!()
                        }
                    });
                } else {
                    error!("Error when receiving message!");
                }
            }
        });
    }
}
