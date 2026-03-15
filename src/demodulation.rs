use rustfft::num_complex::Complex;

pub struct QuadDemod {
    prev_sample: Complex<f32>,
}

impl QuadDemod {
    pub fn new() -> Self {
        Self {
            prev_sample: Complex::new(1.0, 0.0),
        }
    }

    pub fn process(&mut self, signal: &[Complex<f32>]) -> Vec<f32> {
        let mut freq_out = Vec::with_capacity(signal.len());

        for &sample in signal {
            // Δϕ = arg(x[n] * conj(x[n-1]) )
            let delta_phi = (sample * self.prev_sample.conj()).arg();

            freq_out.push(delta_phi);
            self.prev_sample = sample;
        }

        freq_out
    }
}
