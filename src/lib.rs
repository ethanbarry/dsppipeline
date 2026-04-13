use std::{
    alloc::{alloc_zeroed, Layout},
    os::unix::net::UnixStream,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
    },
    thread::sleep,
    time::Duration,
};

pub mod correlation;
pub mod debug_logging;
pub mod demodulation;
pub mod filters;
pub mod processing;

use rtl_sdr_rs::{error::RtlsdrError, RtlSdr, DEFAULT_BUF_LENGTH as BUF_LEN};
use tracing::{debug, error, info, trace, warn};

/// Frequency of the transmitter in Hz.
const FREQUENCY: u32 = 914_975_000;

/// Sample frequency of the RTL-SDR in Hz.
const SAMPLE_FREQUENCY: u32 = 2_048_000;

/// Bandwidth setpoint of the SDR.
const BANDWIDTH: u32 = 80_000;

// Shutdown flag.
pub static TERMINATED: AtomicBool = AtomicBool::new(false);
pub static SAMPLING: AtomicBool = AtomicBool::new(false);

fn open_sdr() -> Result<RtlSdr, ()> {
    if let Ok(sdr) = RtlSdr::open_first_available() {
        Ok(sdr)
    } else {
        error!("No device detected!");
        TERMINATED.store(true, Ordering::Relaxed);
        Err(())
    }
}

/// Configure the SDR device.
pub fn config_sdr(
    sdr: &mut RtlSdr,
    freq: u32,
    sample_freq: u32,
    bandwidth: u32,
) -> Result<(), RtlsdrError> {
    // Use auto-gain
    sdr.set_tuner_gain(rtl_sdr_rs::TunerGain::Auto)?;
    // Disable bias-tee
    sdr.set_bias_tee(false)?;
    // Reset the endpoint before we try to read from it (mandatory)
    sdr.reset_buffer()?;
    // Set the frequency
    sdr.set_center_freq(freq)?;
    // Set sample rate
    sdr.set_sample_rate(sample_freq)?;
    // Set bandwidth
    sdr.set_tuner_bandwidth(bandwidth)?; // 60 kHz
    Ok(())
}

pub fn receive(tx: Sender<Box<[u8; BUF_LEN]>>) {
    info!("Receiving thread started.");
    let sdr = open_sdr();
    if terminated() {
        return;
    }

    let mut sdr = sdr.expect("The device exists.");

    config_sdr(&mut sdr, FREQUENCY, SAMPLE_FREQUENCY, BANDWIDTH)
        .expect("Configuration should work.");

    info!("SDR is tuned to {} Hz.", sdr.get_center_freq());
    info!("Sampling at {} Hz.", sdr.get_sample_rate());

    info!("Reading samples...");
    while !terminated() {
        // if stream
        // if SAMPLING.load(Ordering::Relaxed) {
        let mut ctr = 0;

        while ctr < 24 {
            let mut buf: Box<[u8; BUF_LEN]> = alloc_buf();

            let res = sdr.read_sync(&mut *buf);
            match res {
                Ok(n) => {
                    if n < BUF_LEN {
                        info!("Short read ({:#?}), samples dropped, exiting!", n);
                        break;
                    }
                }
                Err(e) => {
                    info!("Read error: {:#?}", e);
                    break;
                }
            }

            tx.send(buf)
                .expect("The other thread has crashed if this fails.");
            ctr += 1;
        }
        // } else {
        //     sleep(Duration::from_millis(500));
        // }
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
    unsafe {
        let ptr = alloc_zeroed(layout) as *mut T;
        Box::from_raw(ptr)
    }
}
