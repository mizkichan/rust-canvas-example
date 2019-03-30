use std::f64::consts::PI;

#[derive(Debug)]
pub struct Circle {
    pub r: f64,
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub h: u16,
}

impl Circle {
    pub fn gen(r: f64, width: f64, height: f64) -> Circle {
        let (dy, dx) = f64::sin_cos(random() * PI * 2.0);
        Circle {
            r,
            x: random() * width,
            y: random() * height,
            dx,
            dy,
            h: (random() * 360.0) as u16,
        }
    }
}

fn random() -> f64 {
    js_sys::Math::random()
}
