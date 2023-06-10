use std::error;

use colours::{Hsv, Rgb};
use mandelbrot::mandelbrot_color;
use rayon::prelude::*;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Point, render::Canvas, video::Window,
};
mod complex;
mod mandelbrot;

const SCREEN_SIDE: f64 = 1000f64;
const HALF_SCREEN_SIDE: i32 = (SCREEN_SIDE / 2.0) as i32;

fn main() -> Result<(), Box<dyn error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Mandelbrot set", SCREEN_SIDE as u32, SCREEN_SIDE as u32)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().build()?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut zoom = 2.5f64;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => (),
            }
        }
        apply_to_all_pixels(&mut canvas, zoom);
        canvas.present();

        zoom *= 0.9;
    }

    Ok(())
}

fn apply_to_all_pixels(draw: &mut Canvas<Window>, zoom: f64) {
    (-HALF_SCREEN_SIDE..HALF_SCREEN_SIDE)
        .map(|x| (-HALF_SCREEN_SIDE..HALF_SCREEN_SIDE).map(move |y| (x, y)))
        .flatten()
        .par_bridge()
        .into_par_iter()
        .map(|(x, y)| {
            let hsv = mandelbrot_color(zoom, x as f64, y as f64);
            let rgb: Rgb<u8> = hsv_to_rgb(hsv);

            (x + HALF_SCREEN_SIDE, y + HALF_SCREEN_SIDE, rgb)
        })
        .collect::<Vec<_>>()
        .iter()
        .for_each(|(x, y, rgb)| {
            draw.set_draw_color(Color::RGB(rgb.red, rgb.green, rgb.blue));
            draw.draw_point(Point::new(*x, *y)).unwrap();
        });
}

fn hsv_to_rgb(hsv: Hsv<f32>) -> Rgb<u8> {
    Rgb::<u8>::from(Rgb::<f32>::from(hsv))
}
