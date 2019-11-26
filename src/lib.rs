use rand::prelude::*;
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct ArcFillRenderer {
    width: f64,
    height: f64,
    radius: f64,
    circles: Vec<Circle>,
}

#[wasm_bindgen]
impl ArcFillRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64, radius: f64, number_of_circles: usize) -> ArcFillRenderer {
        let mut rng = rand::rngs::OsRng;
        let mut circles = Vec::new();
        circles.resize_with(number_of_circles, || Circle::gen(&mut rng, width, height));
        ArcFillRenderer {
            width,
            height,
            radius,
            circles,
        }
    }

    pub fn update(&mut self) {
        for mut circle in &mut self.circles {
            circle.x += circle.dx;
            circle.y += circle.dy;
            if circle.x < 0.0 || self.width < circle.x {
                circle.dx = -circle.dx
            }
            if circle.y < 0.0 || self.width < circle.y {
                circle.dy = -circle.dy
            }
        }
    }

    pub fn render(&self, context: CanvasRenderingContext2d) {
        context.clear_rect(0.0, 0.0, self.width, self.height);
        for circle in &self.circles {
            context.set_fill_style(&JsValue::from(format!("hsla({},100%,50%,0.5)", circle.h)));
            context.begin_path();
            context
                .arc(circle.x, circle.y, self.radius, 0.0, PI * 2.0)
                .unwrap();
            context.fill();
        }
    }
}

struct Circle {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    h: u16,
}

impl Circle {
    fn gen<R>(rng: &mut R, max_width: f64, max_height: f64) -> Circle
    where
        R: Rng,
    {
        Circle {
            x: rng.gen_range(0.0, max_width),
            y: rng.gen_range(0.0, max_height),
            dx: rng.gen_range(-1.0, 1.0),
            dy: rng.gen_range(-1.0, 1.0),
            h: rng.gen_range(0, 360),
        }
    }
}
