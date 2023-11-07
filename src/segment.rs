//! Line segments from point to point

use crate::core::{ParametricFunction2D, Point, T};

/// A line segment from a start point to an end point
pub struct Segment {
    pub start: Point,
    pub end: Point,
}

impl ParametricFunction2D for Segment {
    fn evaluate(&self, t: T) -> Point {
        let dir = (-self.start.x + self.end.x, -self.start.y + self.end.y);
        let start = self.start;

        (start.x + t.value() * dir.0, start.y + t.value() * dir.1).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_segment() {
        let s = Segment {
            start: (0.0, 0.0).into(),
            end: (1.0, 2.0).into(),
        };

        let res = s.evaluate(T::new(0.5));
        assert_relative_eq!(res.x, 0.5);
        assert_relative_eq!(res.y, 1.0);
    }
}
