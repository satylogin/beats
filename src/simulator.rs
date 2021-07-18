use crate::server::Server;
use std::thread::{self, JoinHandle};

pub(crate) struct Simulator {
    handlers: Vec<JoinHandle<()>>,
}

impl Simulator {
    pub(crate) fn new(n: usize) -> Self {
        Self {
            handlers: (0..n)
                .map(|_| {
                    thread::spawn(|| {
                        Server::new().run();
                    })
                })
                .collect(),
        }
    }
}
