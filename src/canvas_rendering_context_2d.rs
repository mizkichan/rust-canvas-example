use crate::common::Circle;
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
#[derive(Debug)]
pub struct CanvasRenderingContext2dRenderer {
    context: CanvasRenderingContext2d,
    width: f64,
    height: f64,
    radius: f64,
    circles: Vec<Circle>,
}

#[wasm_bindgen]
impl CanvasRenderingContext2dRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new(
        context: CanvasRenderingContext2d,
        width: f64,
        height: f64,
        radius: f64,
    ) -> CanvasRenderingContext2dRenderer {
        CanvasRenderingContext2dRenderer {
            context,
            width,
            height,
            radius,
            circles: Vec::new(),
        }
    }

    #[wasm_bindgen(js_name=setCirclesCount)]
    pub fn set_circles_count(&mut self, n: usize) {
        let &mut Self {
            width,
            height,
            radius,
            ..
        } = self;
        self.circles
            .resize_with(n, || Circle::gen(radius, width, height));
    }

    pub fn update(&mut self) {
        self.context.clear_rect(0.0, 0.0, self.width, self.height);

        for circle in &mut self.circles {
            self.context
                .set_fill_style(&JsValue::from(format!("hsl({},100%,50%,0.5)", circle.h)));

            self.context.begin_path();
            self.context
                .arc(circle.x, circle.y, circle.r, 0.0, PI * 2.0)
                .unwrap();
            self.context.fill();

            circle.x = circle.x + circle.dx;
            if circle.x < 0.0 {
                circle.x = -circle.x;
                circle.dx = -circle.dx;
            } else if self.width < circle.x {
                circle.x = self.width * 2.0 - circle.x;
                circle.dx = -circle.dx;
            }

            circle.y = circle.y + circle.dy;
            if circle.y < 0.0 {
                circle.y = -circle.y;
                circle.dy = -circle.dy;
            } else if self.height < circle.y {
                circle.y = self.height * 2.0 - circle.y;
                circle.dy = -circle.dy;
            }
        }
    }
}
