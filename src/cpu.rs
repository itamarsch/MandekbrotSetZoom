use rayon::prelude::*;
use sdl2::{rect::Point, render::Canvas, video::Window};

use crate::{mandelbrot_color, OFFSET, SCREEN_HEIGHT, SCREEN_RATIO, SCREEN_WIDTH};

use self::complex::Complex;

mod complex;
mod mandelbrot;
const HALF_SCREEN_WIDTH: i32 = (SCREEN_WIDTH / 2.0) as i32;
const HALF_SCREEN_HEIGHT: i32 = (SCREEN_HEIGHT / 2.0) as i32;

#[allow(unused)]
pub fn apply_to_all_pixels_cpu(draw: &mut Canvas<Window>, zoom: f64) {
    (-HALF_SCREEN_WIDTH..HALF_SCREEN_WIDTH)
        .map(|x| (-HALF_SCREEN_HEIGHT..HALF_SCREEN_HEIGHT).map(move |y| (x, y)))
        .flatten()
        .par_bridge()
        .into_par_iter()
        .map(|(x, y)| {
            let c_x = (x as f64 / SCREEN_WIDTH) * (zoom * SCREEN_RATIO);
            let c_y = (y as f64 / SCREEN_HEIGHT) * zoom;
            let iteration = mandelbrot::mandelbrot_iterations(Complex {
                re: c_x + OFFSET.0,
                img: c_y + OFFSET.1,
            });
            let rgb = mandelbrot_color(iteration);

            (x + HALF_SCREEN_WIDTH, y + HALF_SCREEN_HEIGHT, rgb)
        })
        .collect::<Vec<_>>()
        .iter()
        .for_each(|(x, y, rgb)| {
            draw.set_draw_color(*rgb);
            draw.draw_point(Point::new(*x, *y)).unwrap();
        });
}
