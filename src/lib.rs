mod utils;

use std::ops::Add;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    // alert("Hello, photogenic!");
}

#[wasm_bindgen]
pub fn draw_colors(ctx: &CanvasRenderingContext2d, width: u32, height: u32) -> Result<(), JsValue> {
    // Generate a vector of random RGB colors
    let random_colors: Vec<[u8; 3]> = (0..width * height)
        .map(|_| {
            let r = js_sys::Math::floor(js_sys::Math::random() * 256.0) as u8;
            let g = js_sys::Math::floor(js_sys::Math::random() * 256.0) as u8;
            let b = js_sys::Math::floor(js_sys::Math::random() * 256.0) as u8;
            [r, g, b]
        })
        .collect();

    // Calculate the width of each bar
    let bar_width = width as f64 / random_colors.len() as f64;

    // Loop through each RGB color and draw a vertical bar
    for (index, color) in random_colors.iter().enumerate() {
        let [r, g, b] = *color;
        let bar_height = (r as f64 + g as f64 + b as f64) / 3.0; // You can adjust this based on your preference

        // Set the fill style to the current RGB color
        ctx.set_fill_style(&JsValue::from_str(&format!("rgb({}, {}, {})", r, g, b)));

        // Draw the vertical bar
        ctx.fill_rect(
            index as f64 * bar_width,
            height as f64 - bar_height,
            bar_width,
            bar_height,
        );
    }

    Ok(())
}

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    real: f64,
    imaginary: f64,
) -> Result<(), JsValue> {
    // The real workhorse of this algorithm, generating pixel data
    let c = Complex { real, imaginary };
    let data = get_julia_set(width, height, c);
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), width, height)?;
    // ctx.put_image_data(&data, 0.0, 0.0)?;
    // ctx.set_line_width(1.0);

    ctx.move_to(0.0, 0.0);
    ctx.line_to(10.0, 10.0);
    ctx.stroke();

    Ok(())
}

fn get_julia_set(width: u32, height: u32, c: Complex) -> Vec<u8> {
    let mut data = Vec::new();

    let param_i = 1.5;
    let param_r = 1.5;
    let scale = 0.005;

    for x in 0..width {
        for y in 0..height {
            let z = Complex {
                real: y as f64 * scale - param_r,
                imaginary: x as f64 * scale - param_i,
            };
            let iter_index = get_iter_index(z, c);
            data.push((iter_index / 4) as u8);
            data.push((iter_index / 2) as u8);
            data.push(iter_index as u8);
            data.push(255);
        }
    }

    data
}

fn get_iter_index(z: Complex, c: Complex) -> u32 {
    let mut iter_index: u32 = 0;
    let mut z = z;
    while iter_index < 900 {
        if z.norm() > 2.0 {
            break;
        }
        z = z.square() + c;
        iter_index += 1;
    }
    iter_index
}

#[derive(Clone, Copy, Debug)]
struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    fn square(self) -> Complex {
        let real = (self.real * self.real) - (self.imaginary * self.imaginary);
        let imaginary = 2.0 * self.real * self.imaginary;
        Complex { real, imaginary }
    }

    fn norm(&self) -> f64 {
        (self.real * self.real) + (self.imaginary * self.imaginary)
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex {
        Complex {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}
