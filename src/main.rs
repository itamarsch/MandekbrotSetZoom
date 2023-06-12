use std::{cell::RefCell, error, time::Instant};

extern crate ocl;
use ocl::ProQue;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

mod cpu;
#[allow(unused_imports)]
use cpu::apply_to_all_pixels_cpu;

mod gpu;
#[allow(unused_imports)]
use gpu::{apply_to_all_pixels_gpu, GPU_PROGRAM};

const SCREEN_SIDE: f64 = 800f64;
const HALF_SCREEN_SIDE: i32 = (SCREEN_SIDE / 2.0) as i32;

pub const MAX_ITERATIONS: u16 = 2000;
pub const OFFSET: (f64, f64) = (-0.7746806106269039, -0.1374168856037867);
fn main() -> Result<(), Box<dyn error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Mandelbrot set", SCREEN_SIDE as u32, SCREEN_SIDE as u32)
        .position(0, 0)
        .borderless()
        .build()?;

    let mut canvas = window.into_canvas().build()?;

    // These lines are useless when running on CPU
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
        //apply_to_all_pixels_cpu(&mut canvas, zoom);
        println!("Fps : {}", 1.0f32 / now.elapsed().as_secs_f32());
        canvas.present();
        if is_zooming {
            zoom *= 0.95;
        }
    }

    Ok(())
}

pub fn mandelbrot_color(iterations: u16) -> Color {
    if iterations == MAX_ITERATIONS {
        Color::BLACK
    } else {
        palette(iterations)
    }
}
const RGB_PALETTE: [Color; 16] = [
    Color::RGB(66, 30, 15),
    Color::RGB(25, 7, 26),
    Color::RGB(9, 1, 47),
    Color::RGB(4, 4, 73),
    Color::RGB(0, 7, 100),
    Color::RGB(12, 44, 138),
    Color::RGB(24, 82, 177),
    Color::RGB(57, 125, 209),
    Color::RGB(134, 181, 229),
    Color::RGB(211, 236, 248),
    Color::RGB(241, 233, 191),
    Color::RGB(248, 201, 95),
    Color::RGB(255, 170, 0),
    Color::RGB(204, 128, 0),
    Color::RGB(153, 87, 0),
    Color::RGB(106, 52, 3),
];
pub fn palette(iterations: u16) -> Color {
    RGB_PALETTE[iterations as usize % RGB_PALETTE.len()]
}
