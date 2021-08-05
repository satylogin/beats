use crate::server::Server;
use std::thread::{self, JoinHandle};

const IP: &str = "127.0.0.1";
const BASE_PORT: usize = 4000;

pub(crate) struct Simulator {
    handlers: Vec<JoinHandle<()>>,
}

impl Simulator {
    pub(crate) fn new(n: usize) -> Self {
        let server_addrs: Vec<String> = (BASE_PORT..BASE_PORT + n)
            .map(|port| format!("{}:{}", IP, port))
            .collect();
        println!("server addresses: {:#?}", &server_addrs);

        let n = server_addrs.len();
        let handlers = (0..n)
            .map(|i| {
                let server = server_addrs[i].clone();
                let neighbour = server_addrs[(i + 1) % n].clone();
                thread::spawn(move || {
                    Server::new(server, vec![neighbour]).run();
                })
            })
            .collect();
        println!("thread handlers: {:#?}", &handlers);

        Self { handlers }
    }
}
