mod schedular;

use std::sync::Arc;
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

struct HeartBeatState {
    thread_id: thread::ThreadId,
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
        let heartbeat_state = HeartBeatState { thread_id: self.id };
        schedular::schedule(
            |state: Arc<HeartBeatState>| {
                println!("sending heartbeat {:?}", state.thread_id);
            },
            heartbeat_state,
            Duration::from_secs(1),
        );
        loop {
            println!("Currently in thread {:?}", self.id);
            thread::sleep(Duration::from_secs(2));
        }
    }
}
