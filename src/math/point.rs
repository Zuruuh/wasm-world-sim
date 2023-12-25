use std::f64::consts::PI;

use wasm_bindgen::JsValue;

use crate::Drawable;

#[derive(Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub size: usize,
    pub color: String,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            size: 18,
            color: "black".into(),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) -> Result<(), JsValue> {
        let radius = self.size as f64 / 2.0;

        ctx.begin_path();
        ctx.set_fill_style(&self.color.clone().into());
        ctx.arc(self.x, self.y, radius, 0.0, PI * 2.0)?;
        ctx.fill();

        Ok(())
    }
}
