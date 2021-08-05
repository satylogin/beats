use crate::schedular;
use std::sync::Arc;

pub(crate) struct HeartbeatTask;

fn send_heartbeat(addresses: Arc<Vec<String>>) {
    for addr in addresses.iter() {
        // TODO: ping call
        println!("Sending hetbeat to: {:?}", addr);
    }
}

impl HeartbeatTask {
    pub(crate) fn invoke(address: Vec<String>) {
        println!("invoking schedular with addresses: {:#?}", &address);
        schedular::schedule(send_heartbeat, address, std::time::Duration::from_secs(1));
    }
}
