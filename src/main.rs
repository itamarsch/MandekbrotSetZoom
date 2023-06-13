#[cfg(all(feature = "gpu", feature = "cpu"))]
compile_error!("feature \"cpu\" and feature \"gpu\" cannot be enabled at the same time");

use std::{error, time::Instant};

#[cfg(feature = "gpu")]
use std::cell::RefCell;
#[cfg(feature = "gpu")]
extern crate ocl;
#[cfg(feature = "gpu")]
use ocl::ProQue;
#[cfg(feature = "gpu")]
mod gpu;
#[cfg(feature = "gpu")]
use gpu::{apply_to_all_pixels_gpu, GPU_PROGRAM};

use sdl2::{event::Event, keyboard::Keycode};

mod palette;

#[cfg(feature = "cpu")]
mod cpu;
#[cfg(feature = "cpu")]
use cpu::apply_to_all_pixels_cpu;

const SCREEN_HEIGHT: f64 = SCREEN_WIDTH * (1.0 / SCREEN_RATIO);
const SCREEN_WIDTH: f64 = 1000f64;
const SCREEN_RATIO: f64 = 16.0 / 9.0;

pub const MAX_ITERATIONS: u16 = 2000;
pub const OFFSET: (f64, f64) = (-0.7746806106269039, -0.1374168856037867);
fn main() -> Result<(), Box<dyn error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Mandelbrot set", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .borderless()
        .build()?;

    let mut canvas = window.into_canvas().build()?;

    #[cfg(feature = "gpu")]
    let (pro_que, buffer, rust_buffer) = {
        let pro_que = ProQue::builder()
            .src(GPU_PROGRAM)
            .dims((SCREEN_WIDTH, SCREEN_HEIGHT))
            .build()?;
        let buffer = pro_que.create_buffer::<u16>()?;
        let rust_buffer = RefCell::new(vec![0u16; buffer.len()]);
        (pro_que, buffer, rust_buffer)
    };

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
        #[cfg(feature = "gpu")]
        apply_to_all_pixels_gpu(&pro_que, &mut canvas, &buffer, rust_buffer.clone(), zoom)?;
        #[cfg(feature = "cpu")]
        apply_to_all_pixels_cpu(&mut canvas, zoom);
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
