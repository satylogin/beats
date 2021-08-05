mod heartbeat;
mod schedular;
mod server;
mod simulator;

use simulator::Simulator;
use std::thread;
use std::time::Duration;

pub fn simulate(n: usize) {
    println!("starting simulator");
    let _ = Simulator::new(n);
    thread::sleep(Duration::from_secs(3));
    println!("terminating simulation");
}
