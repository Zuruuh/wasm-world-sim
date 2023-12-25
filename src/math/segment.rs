use wasm_bindgen::JsValue;

use crate::Drawable;

use super::Point;

#[derive(Debug)]
pub struct Segment<'graph> {
    pub first: &'graph Point,
    pub second: &'graph Point,
    pub width: usize,
    pub color: String,
}

impl<'graph> Segment<'graph> {
    pub fn new(first: &'graph Point, second: &'graph Point) -> Self {
        Self {
            first,
            second,
            width: 2,
            color: "black".into(),
        }
    }
}

impl<'graph> Drawable for Segment<'graph> {
    fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) -> Result<(), JsValue> {
        ctx.begin_path();
        ctx.set_line_width(self.width as f64);
        ctx.set_stroke_style(&self.color.clone().into());
        ctx.move_to(self.first.x, self.first.y);
        ctx.line_to(self.second.x, self.second.y);
        ctx.stroke();

        Ok(())
    }
}
