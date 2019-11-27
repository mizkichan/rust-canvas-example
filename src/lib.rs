mod arcfill;
mod imagedata;

use image::Rgba;
use palette::{Hsl, LinSrgb, RgbHue};
use rand::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

struct State {
    width: u32,
    height: u32,
    circles: Vec<Circle>,
}

impl State {
    fn new(width: u32, height: u32, number_of_circles: usize) -> State {
        let mut rng = rand::rngs::OsRng;
        let mut circles = Vec::new();
        circles.resize_with(number_of_circles, || {
            Circle::gen(&mut rng, width.into(), height.into())
        });
        State {
            width,
            height,
            circles,
        }
    }

    fn update(&mut self) {
        for mut circle in &mut self.circles {
            circle.x += circle.dx;
            circle.y += circle.dy;
            if circle.x < 0.0 || f64::from(self.width) < circle.x {
                circle.dx = -circle.dx
            }
            if circle.y < 0.0 || f64::from(self.width) < circle.y {
                circle.dy = -circle.dy
            }
        }
    }
}

struct Circle {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    color: Rgba<u8>,
}

impl Circle {
    fn gen<R>(rng: &mut R, max_width: f64, max_height: f64) -> Circle
    where
        R: Rng,
    {
        let hsl = Hsl::new(RgbHue::from_degrees(rng.gen_range(0.0, 360.0)), 1.0, 0.5);
        let rgb = LinSrgb::from(hsl).into_format();
        Circle {
            x: rng.gen_range(0.0, max_width),
            y: rng.gen_range(0.0, max_height),
            dx: rng.gen_range(-1.0, 1.0),
            dy: rng.gen_range(-1.0, 1.0),
            color: Rgba([rgb.red, rgb.green, rgb.blue, 128]),
        }
    }
}
