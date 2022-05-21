#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
mod test;

#[cfg(test)]
#[macro_use]
extern crate approx;

use png::{Encoder, StreamWriter};
use std::fs::File;
use std::io::Error;
use std::io::prelude::*;
use std::io::BufWriter;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

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

        while (x*x + y*y ≤ 2*2 AND iteration < max_iteration) do
            xtemp := x*x - y*y + x0
            y := 2*x*y + y0
            x := xtemp
            iteration := iteration + 1
    
        color := palette[iteration]
        plot(Px, Py, color)
    */
    let mut pixels: [u8; WIDTH * HEIGHT] = [0u8; WIDTH * HEIGHT];
    // pixels[[0,0]] = 8;
    for px in 0 .. WIDTH {
        for py in 0 .. HEIGHT {
            let (x0, y0) = scale(px, py);
            let (mut x, mut y) = (0.0, 0.0);
            //println!("(x0, y0): ({}, {})", x0, y0);
            let mut iteration = 0;
            // let (mut x2, mut y2, mut w) = (0.0, 0.0, 0.0);
            while x * x + y * y <= 4.0 && iteration < MAX_ITERATIONS {
                let xtemp = x*x - y*y + x0;
                y = 2.0 * x * y + y0;
                x = xtemp;
                iteration += 1;
            }
            let color: u8 = iteration.clamp(0, 255) as u8;
            //println!("iteration: {}, color value: {}", iteration, color);
            pixels[(py * WIDTH) + px] = color;
        }
    }

    draw_image(pixels).unwrap();
    println!("File written to output.png");
}

fn scale_intensity(iterations: u32) -> u8 {
    let color_32: u32 = (iterations as f32 * INTENSITY_SCALE).trunc() as u32;
    let color_8: u8 = color_32.try_into().expect("Color went outside the bounds of u8");
    color_8
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

fn draw_image(pixels: [u8; WIDTH *HEIGHT]) -> Result<(), Error> {
    let file_buffer = File::create("output.png")?;
    let mut w = BufWriter::with_capacity(WIDTH * HEIGHT, file_buffer);
    let mut encoder = Encoder::new(w, WIDTH as u32, HEIGHT as u32);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(pixels.as_slice()).unwrap();
    Ok(())
}

