use std::{
    fs::File,
    io::{self, Write},
    sync::mpsc::Receiver,
};

use rtl_sdr_rs::DEFAULT_BUF_LENGTH as BUF_LEN;
use rustfft::{num_complex::Complex, FftNum, FftPlanner};
use tracing::{debug, info, warn};

use crate::{terminated, FREQUENCY, SAMPLE_FREQUENCY};

fn complex_decimate(x: &[Complex<i16>]) -> Vec<Complex<i16>> {
    x.windows(10)
        .step_by(10)
        .map(|zs| {
            zs.iter().fold(Complex::new(0, 0), |acc, z| acc + z) / Complex::new(zs.len() as i16, 0)
        })
        .collect()
}

fn log_complex_to_file(fname: &str, x: &[Complex<i16>]) -> Result<(), io::Error> {
    info!("Opening output file.");
    let mut f = File::create(fname)?;

    x.iter()
        .map(|z| {
            if z.im > 0 {
                format!("{}+{}i\n", z.re, z.im)
            } else if z.im < 0 {
                format!("{}{}i\n", z.re, z.im)
            } else {
                format!("{}+{}i\n", z.re, z.im)
            }
        })
        .for_each(|l| {
            f.write(l.as_bytes()).expect("Write failed!");
        });

    Ok(())
}

pub fn process(rx: Receiver<Box<[u8; BUF_LEN]>>) {
    info!("Processing thread started.");

    while !terminated() {
        let signed_buf = rx
            .recv()
            .expect("The other thread has crashed if this fails.")
            .iter()
            .map(|a| (*a as i16 - 127) / 127)
            .collect::<Vec<i16>>()
            .windows(2)
            .step_by(2)
            .map(|w| Complex::new(w[1], w[0]))
            .collect::<Vec<Complex<i16>>>();

        let signed_buf = complex_decimate(&signed_buf);

        if let Err(_) = log_complex_to_file("output.csv", &signed_buf) {
            warn!("File access error; failed to log values.");
        }
    }
}

/// This should in theory return the delta between the fox's transmitter's frequency and
/// the frequency according to the local oscillator.
fn obtain_freq_offset<T>(_x: &mut [Complex<T>]) -> f32
where
    T: FftNum,
{
    // let bins = 1024;
    // let num_samples = x.len();
    // let mut planner = FftPlanner::new();

    // let fft = planner.plan_fft_forward(bins);
    // fft.process(x);

    // let bins: Vec<(f32, f32)> = x
    //     .iter()
    //     .take(num_samples / 2)
    //     .enumerate()
    //     .map(|(i, z)| (i as f32, (z.norm_sqr() as f32 + f32::EPSILON).log10()))
    //     // .inspect(|(f, p)| {
    //     //     if *f < (FREQUENCY + 100000) as f32 {
    //     //         debug!("{f} Hz: {p} dB")
    //     //     }
    //     // })
    //     .collect();

    // let (max_freq, max_power) = bins
    //     .iter()
    //     .max_by_key(|(_, p)| *p as i32)
    //     .expect("Is okay.");

    // *max_freq
    todo!()
}
