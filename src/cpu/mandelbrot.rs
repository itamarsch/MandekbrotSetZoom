use crate::MAX_ITERATIONS;

use super::complex::Complex;

pub fn mandelbrot_iterations(c: Complex) -> u16 {
    let mut iterations = 0;
    let mut z0 = Complex {
        re: 0f64,
        img: 0f64,
    };

    if cardioid_and_bulb_check(c) {
        return MAX_ITERATIONS;
    }

    loop {
        z0 = z0 * z0 + c;
        let mag_squared = z0.mag_squared();
        iterations += 1;
        if mag_squared > (4f64) || iterations == MAX_ITERATIONS {
            break iterations;
        }
    }
}

fn cardioid_and_bulb_check(c: Complex) -> bool {
    let c_img_sqr = c.img * c.img;
    let c_mag_sqr = c.re * c.re + c_img_sqr;

    return (c.re + 1.0) * (c.re + 1.0) + c_img_sqr <= 0.0625f64
        || c_mag_sqr * (8.0f64 * c_mag_sqr - 3.0) <= 0.09375 - c.re;
}
