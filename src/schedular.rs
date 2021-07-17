use std::sync::Arc;
use std::thread;
use std::time;

pub(crate) fn schedule<I, F>(callback: F, args: I, interval: time::Duration)
where
    F: Fn(Arc<I>) -> () + Send + 'static,
    I: Send + Sync + 'static,
{
    let args = Arc::new(args);
    thread::spawn(move || loop {
        let args = Arc::clone(&args);
        callback(args);
        thread::sleep(interval);
    });
}
