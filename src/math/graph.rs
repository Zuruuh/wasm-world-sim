use std::{collections::BTreeMap, rc::Rc};

use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::Drawable;

use super::{Point, Segment};

#[derive(Debug)]
pub struct Graph {
    index: usize,
    pub points: BTreeMap<PointId, Point>,
    pub segments: Vec<Segment>,
    ctx: Rc<CanvasRenderingContext2d>,
}

#[derive(Eq, PartialEq, Debug, PartialOrd, Ord, Copy, Clone)]
pub struct PointId(pub usize);

impl std::fmt::Display for PointId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Graph {
    pub fn new(ctx: CanvasRenderingContext2d) -> Self {
        Self {
            ctx: Rc::new(ctx),
            index: 0,
            points: BTreeMap::default(),
            segments: Vec::default(),
        }
    }

    pub fn add_point(&mut self, point: Point) -> PointId {
        let id = PointId(self.index);
        self.points.insert(id, point);
        self.index += 1;
        self.draw();

        id
    }

    pub fn create_segment(&mut self, first: PointId, second: PointId) -> Result<(), String> {
        return if !self.points.contains_key(&first) || !self.points.contains_key(&second) {
            Err(format!(
                "Could not link point {first} and {second} since one of them does not exist"
            ))
        } else {
            self.segments.push(Segment::new(first, second));
            self.draw();
            Ok(())
        };
    }

    fn draw(&self) -> Result<(), JsValue> {
        for segment in self.segments.iter() {
            self.ctx.begin_path();
            self.ctx.set_line_width(segment.width as f64);
            self.ctx.set_stroke_style(&segment.color.clone().into());
            let first = self.points.get(&segment.first).unwrap();
            let second = self.points.get(&segment.second).unwrap();

            self.ctx.move_to(first.x, first.y);
            self.ctx.line_to(second.x, second.y);
            self.ctx.stroke();
        }

        for (_, point) in self.points.iter() {
            point.draw(&self.ctx)?;
        }

        Ok(())
    }
}
