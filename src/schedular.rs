use std::sync::Arc;
use std::thread;
use std::time;

pub(crate) fn schedule<I>(
    callback: impl Fn(Arc<I>) -> () + Send + Sync + 'static,
    args: I,
    interval: time::Duration,
) where
    I: Send + Sync + 'static,
{
    let callback = Arc::new(callback);
    let args = Arc::new(args);
    thread::spawn(move || loop {
        let args = Arc::clone(&args);
        Arc::clone(&callback)(args);
        thread::sleep(interval);
    });
}
