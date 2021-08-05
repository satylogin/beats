use crate::heartbeat;
use std::net::{TcpListener, TcpStream};
use std::thread;

#[derive(Clone)]
pub(crate) struct Server {
    id: thread::ThreadId,
    address: String,
    server_addrs: Vec<String>,
}

impl Server {
    pub(crate) fn new(address: String, server_addrs: Vec<String>) -> Self {
        Self {
            id: thread::current().id(),
            address,
            server_addrs,
        }
    }

    pub(crate) fn run(&self) {
        println!("starting server on thread: {:?}", self.id);
        heartbeat::HeartbeatTask::invoke(self.server_addrs.clone());
        let listener = TcpListener::bind(&self.address).unwrap();
        for stream in listener.incoming() {
            let _: TcpStream = stream.unwrap();
        }
    }
}
