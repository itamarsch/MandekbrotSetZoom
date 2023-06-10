use colours::Hsv;

use crate::{complex::Complex, SCREEN_SIDE};

const MAX_ITERATIONS: u32 = 1000;
const OFFSET: (f64, f64) = (-0.761574, -0.0847596);

pub fn mandelbrot_iterations(c: Complex) -> u32 {
    let mut iterations = 0;
    let mut z0 = Complex {
        re: 0f64,
        img: 0f64,
    };
    loop {
        z0 = z0 * z0 + c;
        if z0.mag() > (2f64) || iterations == MAX_ITERATIONS {
            break iterations;
        }
        iterations += 1;
    }
}

pub fn mandelbrot_color(zoom: f64, x: f64, y: f64) -> Hsv<f32> {
    let x = (x / SCREEN_SIDE) * zoom;
    let y = (y / SCREEN_SIDE) * zoom;

    let iterations = mandelbrot_iterations(Complex {
        re: x + OFFSET.0,
        img: y + OFFSET.1,
    });
    if iterations == MAX_ITERATIONS {
        Hsv::new(0f32, 0f32, 0f32)
    } else {
        Hsv::new(iterations as f32 / MAX_ITERATIONS as f32, 1f32, 1f32)
    }
}
