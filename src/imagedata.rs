use crate::State;
use image::RgbaImage;
use imageproc::drawing::Blend;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
pub struct ImageDataRenderer {
    state: State,
    radius: u32,
}

#[wasm_bindgen]
impl ImageDataRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, radius: u32, number_of_circles: usize) -> Self {
        ImageDataRenderer {
            state: State::new(width, height, number_of_circles),
            radius,
        }
    }

    pub fn update(&mut self) {
        self.state.update()
    }

    pub fn render(&self, context: CanvasRenderingContext2d) {
        let mut image = Blend(RgbaImage::new(self.state.width, self.state.height));
        for circle in &self.state.circles {
            imageproc::drawing::draw_filled_circle_mut(
                &mut image,
                (circle.x as i32, circle.y as i32),
                self.radius as i32,
                circle.color,
            );
        }
        let image_data =
            ImageData::new_with_u8_clamped_array(Clamped(&mut image.0), self.state.width).unwrap();
        context.put_image_data(&image_data, 0.0, 0.0).unwrap();
    }
}
