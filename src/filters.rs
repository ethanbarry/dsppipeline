use rustfft::num_complex::Complex;

pub struct BandpassFilter {
    taps: Vec<f32>,
    state: Vec<Complex<f32>>,
}

impl BandpassFilter {
    pub fn new(taps: &[f32]) -> Self {
        let num_taps = taps.len();
        Self {
            taps: taps.to_vec(),
            state: vec![Complex::new(0f32, 0f32); num_taps - 1],
        }
    }

    pub fn default() -> Self {
        Self::new(&BANDPASS1_TAPS)
    }

    pub fn process(&mut self, signal: &mut [Complex<f32>]) {
        let num_taps = self.taps.len();
        let signal_len = signal.len();

        let mut scratch_buffer = Vec::with_capacity(self.state.len() + signal.len());
        scratch_buffer.extend_from_slice(&self.state);
        scratch_buffer.extend_from_slice(signal);

        for i in 0..signal_len {
            let mut sum = Complex::new(0f32, 0f32);

            for (j, &tap) in self.taps.iter().enumerate() {
                sum += scratch_buffer[i + (num_taps - 1) - j] * tap;
            }
            signal[i] = sum;
        }

        if signal_len >= num_taps - 1 {
            self.state
                .copy_from_slice(&signal[signal_len - (num_taps - 1)..]);
        }
    }
}

pub struct BoxcarFilter {
    sum: f32,
    mean: f32,
    idx: usize,
    buf: Vec<f32>,
    width: usize,
}

impl BoxcarFilter {
    pub fn new(width: usize) -> Self {
        Self {
            sum: 0.0,
            mean: 0.0,
            idx: 0,
            buf: vec![0.0; width],
            width,
        }
    }

    pub fn default() -> Self {
        Self::new(10)
    }

    pub fn process(&mut self, signal: &mut [f32]) {
        for &mut val in signal {
            self.sum -= self.buf[self.idx];

            self.sum += val;
            self.buf[self.idx] = todo!()
        }

        todo!()
    }
}

const FIR1_TAPS: [f32; 51] = [
    -0.00093904,
    -0.00091553,
    -0.00092226,
    -0.00091901,
    -0.00084990,
    -0.00064631,
    -0.00023115,
    0.00047578,
    0.00155181,
    0.00306499,
    0.00506793,
    0.00759216,
    0.01064356,
    0.01419924,
    0.01820602,
    0.02258088,
    0.02721331,
    0.03196948,
    0.03669817,
    0.04123788,
    0.04542499,
    0.04910222,
    0.05212708,
    0.05437958,
    0.05576890,
    0.05623842,
    0.05576890,
    0.05437958,
    0.05212708,
    0.04910222,
    0.04542499,
    0.04123788,
    0.03669817,
    0.03196948,
    0.02721331,
    0.02258088,
    0.01820602,
    0.01419924,
    0.01064356,
    0.00759216,
    0.00506793,
    0.00306499,
    0.00155181,
    0.00047578,
    -0.00023115,
    -0.00064631,
    -0.00084990,
    -0.00091901,
    -0.00092226,
    -0.00091553,
    -0.00093904,
];

const BANDPASS1_TAPS: [f32; 52] = [
    -0.00407881,
    -0.00484692,
    -0.00598890,
    -0.00755526,
    -0.00952289,
    -0.01179351,
    -0.01419906,
    -0.01651359,
    -0.01847071,
    -0.01978540,
    -0.02017840,
    -0.01940131,
    -0.01726025,
    -0.01363638,
    -0.00850133,
    -0.00192632,
    0.00591591,
    0.01475672,
    0.02424452,
    0.03396419,
    0.04346167,
    0.05227211,
    0.05994939,
    0.06609507,
    0.07038454,
    0.07258857,
    0.07258857,
    0.07038454,
    0.06609507,
    0.05994939,
    0.05227211,
    0.04346167,
    0.03396419,
    0.02424452,
    0.01475672,
    0.00591591,
    -0.00192632,
    -0.00850133,
    -0.01363638,
    -0.01726025,
    -0.01940131,
    -0.02017840,
    -0.01978540,
    -0.01847071,
    -0.01651359,
    -0.01419906,
    -0.01179351,
    -0.00952289,
    -0.00755526,
    -0.00598890,
    -0.00484692,
    -0.00407881,
];
