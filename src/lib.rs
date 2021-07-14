use std::thread::{self, JoinHandle};
use std::time::Duration;

pub fn simulate(n: usize) {
    let _ = Simulator::new(n);
    thread::sleep(Duration::from_secs(5));
}

struct Simulator {
    handlers: Vec<JoinHandle<()>>,
}

impl Simulator {
    fn new(n: usize) -> Self {
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

#[derive(Clone)]
struct Server {
    id: thread::ThreadId,
}

impl Server {
    fn new() -> Self {
        Self {
            id: thread::current().id(),
        }
    }

    fn run(&self) {
        loop {
            println!("Currently in thread {:?}", self.id);
            thread::sleep(Duration::from_secs(2));
        }
    }
}
