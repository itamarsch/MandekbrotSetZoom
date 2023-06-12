use rayon::prelude::*;
use sdl2::{rect::Point, render::Canvas, video::Window};

use crate::{mandelbrot_color, HALF_SCREEN_SIDE, OFFSET, SCREEN_SIDE};

use self::complex::Complex;

mod complex;
mod mandelbrot;

#[allow(unused)]
pub fn apply_to_all_pixels_cpu(draw: &mut Canvas<Window>, zoom: f64) {
    (-HALF_SCREEN_SIDE..HALF_SCREEN_SIDE)
        .map(|x| (-HALF_SCREEN_SIDE..HALF_SCREEN_SIDE).map(move |y| (x, y)))
        .flatten()
        .par_bridge()
        .into_par_iter()
        .map(|(x, y)| {
            let c_x = (x as f64 / SCREEN_SIDE) * zoom;
            let c_y = (y as f64 / SCREEN_SIDE) * zoom;
            let iteration = mandelbrot::mandelbrot_iterations(Complex {
                re: c_x + OFFSET.0,
                img: c_y + OFFSET.1,
            });
            let rgb = mandelbrot_color(iteration);

            (x + HALF_SCREEN_SIDE, y + HALF_SCREEN_SIDE, rgb)
        })
        .collect::<Vec<_>>()
        .iter()
        .for_each(|(x, y, rgb)| {
            draw.set_draw_color(*rgb);
            draw.draw_point(Point::new(*x, *y)).unwrap();
        });
}
