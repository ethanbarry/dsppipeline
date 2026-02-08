use std::{
    alloc::{alloc_zeroed, Layout},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
    },
};

pub mod processing;

use rtl_sdr_rs::{RtlSdr, DEFAULT_BUF_LENGTH as BUF_LEN};
use tracing::{debug, error, info, trace, warn};

use crate::processing::config_sdr;

/// Frequency of the transmitter in Hz.
const FREQUENCY: u32 = 914_975_000;

/// Sample frequency of the RTL-SDR in Hz.
const SAMPLE_FREQUENCY: u32 = 2_048_000;

// Shutdown flag.
pub static TERMINATED: AtomicBool = AtomicBool::new(false);

fn open_sdr() -> Result<RtlSdr, ()> {
    if let Ok(sdr) = RtlSdr::open_first_available() {
        Ok(sdr)
    } else {
        error!("No device detected!");
        TERMINATED.store(true, Ordering::Relaxed);
        Err(())
    }
}

pub fn receive(tx: Sender<Vec<u8>>) {
    info!("Receiving thread started.");
    let sdr = open_sdr();
    if terminated() {
        return;
    }

    let mut sdr = sdr.expect("The device exists.");

    config_sdr(&mut sdr, FREQUENCY, SAMPLE_FREQUENCY).expect("Configuration should work.");

    info!("SDR is tuned to {} Hz.", sdr.get_center_freq());
    info!("Sampling at {} Hz.", sdr.get_sample_rate());

    info!("Reading samples...");
    while !terminated() {
        let mut buf: Box<[u8; BUF_LEN]> = alloc_buf();

        let n = sdr.read_sync(&mut *buf);
        if n.is_err() {
            info!("Read error: {:#?}", n);
            break;
        }

        let len = n.expect("n is not an err");
        if len < BUF_LEN {
            info!("Short read ({:#?}), samples dropped, exiting!", len);
            break;
        }

        tx.send(buf.to_vec())
            .expect("The other thread has crashed if this fails.");
    }

    warn!("Closing SDR!");
    sdr.close().unwrap();
}

fn terminated() -> bool {
    TERMINATED.load(Ordering::Relaxed)
}

/// Allocate a buffer on the heap
fn alloc_buf<T>() -> Box<T> {
    let layout: Layout = Layout::new::<T>();
    // TODO move to using safe code once we can allocate an array directly on the heap.
    unsafe {
        let ptr = alloc_zeroed(layout) as *mut T;
        Box::from_raw(ptr)
    }
}
