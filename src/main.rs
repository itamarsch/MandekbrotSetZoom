use std::{cell::RefCell, error, time::Instant};
extern crate ocl;
use colours::{Hsv, Rgb};
use gpu::GPU_PROGRAM;
use ocl::ProQue;
use sdl2::{event::Event, keyboard::Keycode};

use crate::gpu::apply_to_all_pixels_gpu;
mod cpu;
mod gpu;
const SCREEN_SIDE: f64 = 800f64;
const HALF_SCREEN_SIDE: i32 = (SCREEN_SIDE / 2.0) as i32;

pub const MAX_ITERATIONS: u16 = 2000;
pub const OFFSET: (f64, f64) = (-0.7746806106269039, -0.1374168856037867);
fn main() -> Result<(), Box<dyn error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Mandelbrot set", SCREEN_SIDE as u32, SCREEN_SIDE as u32)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().build()?;

    let pro_que = ProQue::builder()
        .src(GPU_PROGRAM)
        .dims((SCREEN_SIDE, SCREEN_SIDE))
        .build()?;
    let buffer = pro_que.create_buffer::<u16>()?;
    let rust_buffer = RefCell::new(vec![0u16; buffer.len()]);

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
        let now = Instant::now();
        apply_to_all_pixels_gpu(&pro_que, &mut canvas, &buffer, rust_buffer.clone(), zoom)?;
        println!("Fps : {}", 1.0f32 / now.elapsed().as_secs_f32());
        canvas.present();

        zoom *= 0.9;
    }

    Ok(())
}

fn hsv_to_rgb(hsv: Hsv<f32>) -> Rgb<u8> {
    Rgb::<u8>::from(Rgb::<f32>::from(hsv))
}

pub fn mandelbrot_color(iterations: u16) -> Hsv<f32> {
    if iterations == MAX_ITERATIONS {
        Hsv::new(0f32, 0f32, 0f32)
    } else {
        Hsv::new(
            (0.65 + iterations as f32 / MAX_ITERATIONS as f32) % 1.0, // Creates prettier gradient
            1f32,
            1f32,
        )
    }
}
