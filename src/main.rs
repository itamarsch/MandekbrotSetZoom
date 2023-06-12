use std::{cell::RefCell, error, time::Instant};
extern crate ocl;
use colours::Rgb;
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
    let mut is_zooming = false;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => is_zooming = !is_zooming,
                _ => (),
            }
        }
        let now = Instant::now();
        apply_to_all_pixels_gpu(&pro_que, &mut canvas, &buffer, rust_buffer.clone(), zoom)?;
        println!("Fps : {}", 1.0f32 / now.elapsed().as_secs_f32());
        canvas.present();
        if is_zooming {
            zoom *= 0.95;
        }
    }

    Ok(())
}

pub fn mandelbrot_color(iterations: u16) -> Rgb<u8> {
    if iterations == MAX_ITERATIONS {
        Rgb::new(0, 0, 0)
    } else {
        palette(iterations)
    }
}
pub fn palette(iterations: u16) -> Rgb<u8> {
    let rgb_palette = [
        Rgb::new(66, 30, 15),
        Rgb::new(25, 7, 26),
        Rgb::new(9, 1, 47),
        Rgb::new(4, 4, 73),
        Rgb::new(0, 7, 100),
        Rgb::new(12, 44, 138),
        Rgb::new(24, 82, 177),
        Rgb::new(57, 125, 209),
        Rgb::new(134, 181, 229),
        Rgb::new(211, 236, 248),
        Rgb::new(241, 233, 191),
        Rgb::new(248, 201, 95),
        Rgb::new(255, 170, 0),
        Rgb::new(204, 128, 0),
        Rgb::new(153, 87, 0),
        Rgb::new(106, 52, 3),
    ];
    rgb_palette[iterations as usize % rgb_palette.len()]
}
