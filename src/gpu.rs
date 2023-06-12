use std::{cell::RefCell, ops::DerefMut};

use colours::Rgb;
use ocl::{Buffer, ProQue};
use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

pub const GPU_PROGRAM: &'static str = include_str!("./gpu/mandelbrot.ocl");

use crate::{hsv_to_rgb, mandelbrot_color, HALF_SCREEN_SIDE, MAX_ITERATIONS, OFFSET, SCREEN_SIDE};

pub fn apply_to_all_pixels_gpu(
    pro_que: &ProQue,
    draw: &mut Canvas<Window>,
    buffer: &Buffer<u16>,
    rust_buffer: RefCell<Vec<u16>>,
    zoom: f64,
) -> ocl::Result<()> {
    let kernel = pro_que
        .kernel_builder("add")
        .arg(buffer)
        .arg(MAX_ITERATIONS)
        .arg(SCREEN_SIDE)
        .arg(HALF_SCREEN_SIDE)
        .arg(zoom)
        .arg(OFFSET.0)
        .arg(OFFSET.1)
        .build()?;

    unsafe {
        kernel.enq()?;
    }
    let mut mut_borrow_buffer = rust_buffer.borrow_mut();
    buffer.read(mut_borrow_buffer.deref_mut()).enq()?;
    drop(mut_borrow_buffer);

    rust_buffer
        .borrow()
        .iter()
        .enumerate()
        .for_each(|(index, iterations)| {
            let y = index / SCREEN_SIDE as usize;
            let x = index % SCREEN_SIDE as usize;

            let hsv = mandelbrot_color(*iterations);
            let rgb: Rgb<u8> = hsv_to_rgb(hsv);
            draw.set_draw_color(Color::RGB(rgb.red, rgb.green, rgb.blue));
            draw.draw_point(Point::new(x as i32, y as i32)).unwrap();
        });
    Ok(())
}