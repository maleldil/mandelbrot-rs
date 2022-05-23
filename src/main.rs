#![allow(dead_code)]
mod test;
mod image;
use image::Image;

#[cfg_attr(test, macro_use)]
extern crate approx;

const WIDTH: usize = 1235;
const HEIGHT: usize = 1120;

// Actual constants
const MIN_X: f64 = -2.0;
const MAX_X: f64 = 0.47;
const MIN_Y: f64 = -1.12;
const MAX_Y: f64 = 1.12;
const X_DIST: f64 = MAX_X - MIN_X;
const Y_DIST: f64 = MAX_Y - MIN_Y;
const MAX_ITERATIONS: u32 = 1000;
const COLOR_DEPTH: u8 = 255;
const INTENSITY_SCALE: f32 = COLOR_DEPTH as f32 / MAX_ITERATIONS as f32;



fn main() {
    // Create a grid over the complex plane between (-2.00, 0.47) on x and (-1.12, 1.12) on y
    // for each sample point z on the grid, perform a number of iterations
    // to determine a) whether the point trends towards infinity and b) if so, at what speed (magnitude)
    // if point does not trend toward infinity, color the pixel represented by the point as black
    // otherwise color it based on its magnitude
    // when you have your grid of sample points along with colors, draw them to a bitmap

    /*
    for each pixel (Px, Py) on the screen do
        x0 := scaled x coordinate of pixel (scaled to lie in the Mandelbrot X scale (-2.00, 0.47))
        y0 := scaled y coordinate of pixel (scaled to lie in the Mandelbrot Y scale (-1.12, 1.12))
        x := 0.0
        y := 0.0
        iteration := 0
        max_iteration := 1000
        x2 := 0
        y2 := 0
        w := 0

        while (x2 + y2 ≤ 4 and iteration < max_iteration) do
            y := 2 × x × y + y0
            x := x2 - y2 + x0
            x2 := x × x
            y2 := y × y
            iteration := iteration + 1
        color := palette[iteration]
        plot(Px, Py, color)
    */
    let mut pixels: [u8; WIDTH * HEIGHT] = [0u8; WIDTH * HEIGHT];

    for px in 0 .. WIDTH {
        for py in 0 .. HEIGHT {
            let (x0, y0) = scale(px, py);
            let (mut x, mut y) = (0.0, 0.0);
            let (mut x2, mut y2) = (0.0, 0.0);
            let mut iteration = 0;

            while x2 + y2 <= 4.0 && iteration < MAX_ITERATIONS {
                y = 2.0 * x * y + y0;
                x = x2 - y2 + x0;
                x2 = x * x;
                y2 = y * y;
                iteration += 1;
            }
            // if we actually scale these over 1000 instead we don't get good looking output
            let color: u8 = iteration.clamp(0, 255) as u8;
            pixels[(py * WIDTH) + px] = color;
        }
    }
    let image = Image::new("output.png".to_string(), WIDTH, HEIGHT);
    image.draw_image(&pixels).unwrap();
    println!("File written to output.png");
}

fn scale(x: usize, y: usize) -> (f64, f64) {
    // for x we want 0 to = -2.0 and max_width = 0.47
    // so we need to multiply X * (2.47 / num_steps)
    let dx = (x as f64 * X_DIST / (WIDTH - 1) as f64) + MIN_X;
    // for y we want 0 to = -1.12 and max_height = 1.12
    // so we need to multiply Y * (2.24 / num_steps)
    // we subtract 1 since we're starting counting from zero
    let dy = (y as f64 * Y_DIST / (HEIGHT - 1) as f64) + MIN_Y;
    (dx, dy)
}