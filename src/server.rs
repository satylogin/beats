use crate::schedular;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct HeartBeatState {
    thread_id: thread::ThreadId,
}

#[derive(Clone)]
pub(crate) struct Server {
    id: thread::ThreadId,
}

impl Server {
    pub(crate) fn new() -> Self {
        Self {
            id: thread::current().id(),
        }
    }

    pub(crate) fn run(&self) {
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
