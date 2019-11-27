use crate::State;
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub struct ArcFillRenderer {
    state: State,
    radius: u32,
}

#[wasm_bindgen]
impl ArcFillRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, radius: u32, number_of_circles: usize) -> Self {
        ArcFillRenderer {
            state: State::new(width, height, number_of_circles),
            radius,
        }
    }

    pub fn update(&mut self) {
        self.state.update()
    }

    pub fn render(&self, context: CanvasRenderingContext2d) {
        context.clear_rect(0.0, 0.0, self.state.width.into(), self.state.height.into());
        for circle in &self.state.circles {
            let color = format!(
                "rgba({},{},{},{}%)",
                circle.color.0[0],
                circle.color.0[1],
                circle.color.0[2],
                (f64::from(circle.color.0[3]) / 256.0 * 100.0),
            );
            context.set_fill_style(&JsValue::from(color));
            context.begin_path();
            context
                .arc(circle.x, circle.y, self.radius.into(), 0.0, PI * 2.0)
                .unwrap();
            context.fill();
        }
    }
}
