use super::PointId;

#[derive(Debug)]
pub struct Segment {
    pub first: PointId,
    pub second: PointId,
    pub width: usize,
    pub color: String,
}

impl Segment {
    pub fn new(first: PointId, second: PointId) -> Self {
        Self {
            first,
            second,
            width: 2,
            color: "black".into(),
        }
    }
}
