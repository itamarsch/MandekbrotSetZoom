use std::{cell::RefCell, error, time::Instant};

extern crate ocl;
use ocl::ProQue;

use sdl2::{event::Event, keyboard::Keycode};

mod palette;
use palette::mandelbrot_color;

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
                Event::Quit { .. } | keyDown!(Keycode::Escape) => break 'main,
                keyDown!(Keycode::Space) => is_zooming = !is_zooming,
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

#[macro_export]
macro_rules! keyDown {
    ($keycode:path) => {
        Event::KeyDown {
            keycode: Some($keycode),
            ..
        }
    };
}
