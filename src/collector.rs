use std::borrow::Borrow;
use std::default::Default;
use crate::info;

pub enum CollectorTarget {
    Buyee
}

pub struct Collector {
    targets: [CollectorTarget; 1],
}

impl Default for Collector {
    fn default() -> Self {
        Self {
            targets: [CollectorTarget::Buyee],
        }
    }
}

impl Collector {
    pub fn new() -> Collector {
        Collector {
            ..Default::default()
        }
    }

    pub fn collect_from_all(&self) {
        for target in &self.targets {
            match target {
                CollectorTarget::Buyee => info!("Handling buyee scrape")
            }
        }
    }
}
