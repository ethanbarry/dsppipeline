use std::{
    os::unix::net::UnixStream,
    process::exit,
    sync::{atomic::Ordering, mpsc},
    thread::{self, sleep},
    time::Duration,
};

use dsppipeline::{processing::process, receive, SAMPLING, TERMINATED};
use tracing::{debug, error, info, trace, warn, Level};

fn main() {
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(Level::TRACE)
        .init();

    info!("Starting...");

    // Set a handler for Ctrl-C. This flag is checked during long-running computations.
    ctrlc::set_handler(|| {
        warn!("Termination scheduled!");
        TERMINATED.store(true, Ordering::Relaxed);
    })
    .expect("Ctrl-C handler could not be set; terminating.");

    //  let stream = UnixStream::connect("/tmp/commsocket").expect("This should not fail.");

    let (tx, rx) = mpsc::channel();

    let receive_thread = thread::spawn(|| receive(tx));
    let process_thread = thread::spawn(|| process(rx));

    receive_thread.join().expect("IS OKAY?");
    process_thread.join().expect("IS OKAY?");
}
