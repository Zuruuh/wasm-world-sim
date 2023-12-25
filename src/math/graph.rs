use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::Drawable;

use super::{Point, Segment};

#[derive(Default, Debug)]
pub struct Graph<'points> {
    pub points: Vec<&'points Point>,
    pub segments: Vec<Segment<'points>>,
}

impl<'a> Graph<'a> {
    pub fn add_points(&mut self, points: Vec<&'a Point>) {
        self.points.append(&mut points.clone());
    }

    pub fn add_point(&mut self, point: &'a Point) {
        self.points.push(point);
    }

    pub fn create_segment(&mut self, first: &'a Point, second: &'a Point) {
        self.segments.push(Segment::new(first, second));
    }
}

impl<'a> Drawable for Graph<'a> {
    fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        for segment in &self.segments {
            segment.draw(ctx)?;
        }

        for point in &self.points {
            point.draw(ctx)?;
        }

        Ok(())
    }
}
