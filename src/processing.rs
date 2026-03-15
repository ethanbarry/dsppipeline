use std::sync::mpsc::Receiver;

use rtl_sdr_rs::DEFAULT_BUF_LENGTH as BUF_LEN;
use rustfft::{num_complex::Complex, FftNum, FftPlanner};
use tracing::{debug, info, warn};

use crate::{
    debug_logging::log_complex_floats_to_file,
    demodulation::QuadDemod,
    filters::{BandpassFilter, BoxcarFilter},
    terminated, FREQUENCY, SAMPLE_FREQUENCY,
};

fn complex_decimate(x: &[Complex<i16>]) -> Vec<Complex<i16>> {
    x.windows(10)
        .step_by(10)
        .map(|zs| {
            zs.iter().fold(Complex::new(0, 0), |acc, z| acc + z) / Complex::new(zs.len() as i16, 0)
        })
        .collect()
}

fn absolute_power(x: &[Complex<f32>]) -> f32 {
    let sum_sq: f32 = x.iter().map(|z| z.norm_sqr()).sum();
    sum_sq / x.len() as f32
}

pub fn process(rx: Receiver<Box<[u8; BUF_LEN]>>) {
    info!("Processing thread started.");

    while !terminated() {
        let signal = rx
            .recv() // Await data from the other thread.
            .expect("The other thread has crashed if this fails.")
            .iter()
            .map(|&a| (a as i16 - 127) / 127) // Unsigned to signed conversion.
            .collect::<Vec<i16>>()
            .windows(2)
            .step_by(2) // Grab the interleaved (I, Q) pairs.
            .map(|w| Complex::new(w[1], w[0]))
            .collect::<Vec<Complex<i16>>>();

        let signal = complex_decimate(&signal); // Downsample with a moving average filter.

        let mut signal: Vec<Complex<f32>> = signal
            .iter()
            .map(|&z| Complex::new(z.re as f32, z.im as f32))
            .collect();

        let power = absolute_power(&signal);
        // info!("Opening output file.");
        // log_complex_floats_to_file("cmplxOutputPre.csv", &float_signal).expect("File OK.");

        let mut filter = BandpassFilter::default();
        filter.process(&mut signal);

        let filtered_power = absolute_power(&signal);
        let snr_db = 10.0 * (filtered_power / power + f32::EPSILON).log10();

        info!("SNR: {snr_db} dB");

        log_complex_floats_to_file("cmplxOutputPost.csv", &signal).expect("File OK.");

        let mut demod = QuadDemod::new();
        let datastream = demod.process(&signal);

        // Boxcar filter the datastream (overlapping moving average).
        let datastream = datastream
            .windows(1000)
            .step_by(500)
            .map(|t| t.iter().sum::<f32>() / t.len() as f32)
            .collect::<Vec<f32>>();

        /*
           Okay, our sample rate was 2048000 Hz on the RTL-SDR.
           We downsampled once for the decimation step, so the new sample rate is 2048000 / 10 = 204800 Hz.
           We have just downsampled once more, but in a more complicated way.

           Every 500 samples just got averaged with their neighbors into one singular value. Then the
           downsampling was 500-to-1 and our new sample rate is 204800 / 500 = 409.6 Hz.
        */

        // Zero-crossing Bit Transition won't work by itself since that would show [1 1 1] as a single [1].
        // Time info is necessary.
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
