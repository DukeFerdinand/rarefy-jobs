use std::thread;
use std::thread::sleep;
use std::time::Duration;
use crossbeam::channel::{Sender, Receiver};

use crate::info;


pub struct Scraper {
    transmitter: Sender<()>,
    receiver: Receiver<()>
}

impl Scraper {
    pub fn new() -> Scraper {
        let (tx, rx) = crossbeam::channel::unbounded();

        Scraper {
            transmitter: tx,
            receiver: rx
        }
    }

    pub fn run(&mut self) {
        thread::scope(|thread_scope| {
            loop {
                info!("hmmm");
                sleep(Duration::from_secs(10));
            }
        });
    }
}
