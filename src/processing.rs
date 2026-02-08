use std::sync::mpsc::Receiver;

use rtl_sdr_rs::{error::RtlsdrError, RtlSdr};
use rustfft::num_complex::Complex;
use tracing::info;

use crate::{terminated, SAMPLE_FREQUENCY};

pub fn process(rx: Receiver<Vec<u8>>) {
    use std::fs::File;
    use std::io::prelude::*;

    info!("Opening output file.");
    let mut f = File::create("./output.csv").expect("File I/O error!");

    info!("Processing thread started.");

    let mut buf = rx
        .recv()
        .expect("The other thread has crashed if this fails.");

    while !terminated() {
        // 1. Rotate the buffer 90 degrees in the complex plane.
        let mut tmp: u8;
        for i in (0..buf.len()).step_by(8) {
            tmp = 255 - buf[i + 3];
            buf[i + 3] = buf[i + 2];
            buf[i + 3] = tmp;

            buf[i + 4] = 255 - buf[i + 4];
            buf[i + 5] = 255 - buf[i + 5];

            tmp = 255 - buf[i + 6];
            buf[i + 6] = buf[i + 7];
            buf[i + 7] = tmp;
        }

        let signed_buf = buf
            .iter()
            .map(|a| *a as i16 - 127)
            .collect::<Vec<i16>>()
            .windows(2)
            .step_by(2)
            .map(|w| Complex::new(w[0] as i32, w[1] as i32))
            .collect::<Vec<Complex<i32>>>();

        let mut res = vec![];
        // Low-pass filter here.
        let mut lp_now = Complex::new(0, 0);
        let mut prev_idx = 0;
        for orig in 0..signed_buf.len() {
            lp_now += buf[orig];

            prev_idx += 1;
            if prev_idx < (1_000_000 / SAMPLE_FREQUENCY + 1) as usize {
                continue;
            }

            res.push(lp_now);
            lp_now = Complex::new(0, 0);
            prev_idx = 0;
        }

        let string_buf = signed_buf
            .iter()
            .map(|z| format!("{},{}\n", z.re, z.im))
            .collect::<Vec<String>>();

        string_buf.iter().for_each(|l| {
            f.write(l.as_bytes()).expect("Write failed!");
        });
    }
}

/// Configure the SDR device.
pub fn config_sdr(sdr: &mut RtlSdr, freq: u32, sample_freq: u32) -> Result<(), RtlsdrError> {
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
    // Set bandwidth?
    sdr.set_tuner_bandwidth(70_000)?; // 70 kHz
    Ok(())
}
