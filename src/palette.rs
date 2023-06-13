use sdl2::pixels::Color;

use crate::MAX_ITERATIONS;

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

pub fn mandelbrot_color(iterations: u16) -> Color {
    if iterations == MAX_ITERATIONS {
        Color::BLACK
    } else {
        palette(iterations)
    }
}
fn palette(iterations: u16) -> Color {
    RGB_PALETTE[iterations as usize % RGB_PALETTE.len()]
}
