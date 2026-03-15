use std::fs::OpenOptions;
use std::io::{self, Write};

use rustfft::num_complex::Complex;

pub fn log_complex_to_file(fname: &str, x: &[Complex<i16>]) -> Result<(), io::Error> {
    let mut f = OpenOptions::new().append(true).create(true).open(fname)?;

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

pub fn log_complex_floats_to_file(fname: &str, x: &[Complex<f32>]) -> Result<(), io::Error> {
    let mut f = OpenOptions::new().append(true).create(true).open(fname)?;

    x.iter()
        .map(|z| {
            if z.im > 0.0 {
                format!("{:.8}+{:.8}i\n", z.re, z.im)
            } else if z.im < 0.0 {
                format!("{:.8}{:.8}i\n", z.re, z.im)
            } else {
                format!("{:.8}+{:.8}i\n", z.re, z.im)
            }
        })
        .for_each(|l| {
            f.write(l.as_bytes()).expect("Write failed!");
        });

    Ok(())
}
