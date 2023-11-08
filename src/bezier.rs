//! Bezier curves

use std::rc::Rc;

use crate::{core::Concat, core::ParametricFunction2D, core::Point, core::T, segment::Segment};

/// Second Order Bezier curve
pub struct BezierSecond {
    pub start: Point,
    pub end: Point,
    pub control: Point,
}

impl BezierSecond {
    pub fn new(start: Point, end: Point, control: Point) -> Self {
        Self {
            start,
            end,
            control,
        }
    }
}

impl ParametricFunction2D for BezierSecond {
    fn evaluate(&self, t: T) -> Point {
        let start = self.start;
        let end = self.end;
        let control = self.control;

        let start_control = Segment {
            start,
            end: control,
        };
        let control_end = Segment {
            start: control,
            end,
        };

        let t1 = start_control.evaluate(t);
        let t2 = control_end.evaluate(t);

        let s = Segment { start: t1, end: t2 };
        s.evaluate(t)
    }
}

/// Third Order Bezier curve
pub struct BezierThird {
    pub start: Point,
    pub end: Point,
    pub control1: Point,
    pub control2: Point,
}

impl BezierThird {
    pub fn new(start: Point, end: Point, control1: Point, control2: Point) -> Self {
        Self {
            start,
            end,
            control1,
            control2,
        }
    }
}

impl ParametricFunction2D for BezierThird {
    fn evaluate(&self, t: T) -> Point {
        let start = self.start;
        let end = self.end;
        let control1 = self.control1;
        let control2 = self.control2;

        let start_control1 = Segment {
            start,
            end: control1,
        };
        let control1_control2 = Segment {
            start: control1,
            end: control2,
        };
        let control2_end = Segment {
            start: control2,
            end,
        };

        let t1 = start_control1.evaluate(t);
        let t2 = control1_control2.evaluate(t);
        let t3 = control2_end.evaluate(t);

        let b = BezierSecond {
            start: t1,
            control: t2,
            end: t3,
        };

        b.evaluate(t)
    }
}

/// Fourth Order Bezier curve
pub struct BezierFourth {
    pub start: Point,
    pub end: Point,
    pub control1: Point,
    pub control2: Point,
    pub control3: Point,
}

impl BezierFourth {
    pub fn new(
        start: Point,
        end: Point,
        control1: Point,
        control2: Point,
        control3: Point,
    ) -> Self {
        Self {
            start,
            end,
            control1,
            control2,
            control3,
        }
    }
}

impl ParametricFunction2D for BezierFourth {
    fn evaluate(&self, t: T) -> Point {
        let start = self.start;
        let end = self.end;
        let control1 = self.control1;
        let control2 = self.control2;
        let control3 = self.control3;

        let start_control1 = Segment {
            start,
            end: control1,
        };
        let control1_control2 = Segment {
            start: control1,
            end: control2,
        };
        let control2_control3 = Segment {
            start: control2,
            end: control3,
        };
        let control3_end = Segment {
            start: control3,
            end,
        };

        let t1 = start_control1.evaluate(t);
        let t2 = control1_control2.evaluate(t);
        let t3 = control2_control3.evaluate(t);
        let t4 = control3_end.evaluate(t);

        let b = BezierThird {
            start: t1,
            control1: t2,
            control2: t3,
            end: t4,
        };

        b.evaluate(t)
    }
}

/// Second Order Bezier spline
pub struct BezierSecondSpline {
    pub points: Vec<Point>,
}

impl BezierSecondSpline {
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }
}

impl ParametricFunction2D for BezierSecondSpline {
    fn evaluate(&self, t: T) -> Point {
        let step = 2;
        let bs: Vec<_> = self
            .points
            .windows(3)
            .enumerate()
            .filter(|&(i, _)| i % step == 0)
            .map(|(_, t)| {
                let t = t.to_vec();
                Rc::new(Box::new(BezierSecond {
                    start: t[0],
                    end: t[2],
                    control: t[1],
                }) as Box<dyn ParametricFunction2D>)
            })
            .collect();

        let concat = Concat { functions: bs };
        concat.evaluate(t)
    }
}

/// Third Order Bezier spline
pub struct BezierThirdSpline {
    pub points: Vec<Point>,
}

impl BezierThirdSpline {
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }
}

impl ParametricFunction2D for BezierThirdSpline {
    fn evaluate(&self, t: T) -> Point {
        let step = 3;
        let bs: Vec<_> = self
            .points
            .windows(4)
            .enumerate()
            .filter(|&(i, _)| i % step == 0)
            .map(|(_, t)| {
                let t = t.to_vec();
                Rc::new(Box::new(BezierThird {
                    start: t[0],
                    end: t[3],
                    control1: t[1],
                    control2: t[2],
                }) as Box<dyn ParametricFunction2D>)
            })
            .collect();

        let concat = Concat { functions: bs };
        concat.evaluate(t)
    }
}

/// Fourth Order Bezier spline
pub struct BezierFourthSpline {
    pub points: Vec<Point>,
}

impl BezierFourthSpline {
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }
}

impl ParametricFunction2D for BezierFourthSpline {
    fn evaluate(&self, t: T) -> Point {
        let step = 4;
        let bs: Vec<_> = self
            .points
            .windows(5)
            .enumerate()
            .filter(|&(i, _)| i % step == 0)
            .map(|(_, t)| {
                Rc::new(Box::new(BezierFourth {
                    start: t[0],
                    end: t[4],
                    control1: t[1],
                    control2: t[2],
                    control3: t[3],
                }) as Box<dyn ParametricFunction2D>)
            })
            .collect();

        let concat = Concat { functions: bs };
        concat.evaluate(t)
    }
}

// THIS IS PROBABLY POSSIBLE!! Lets Stop at 4th order for now!

// struct BezierNth<const N: usize> {
//     points: [(f32, f32); N],
// }

// impl<const N: usize> ParametricFunction2D for BezierNth<N> {
//     fn calculate(&self, t: T) -> (f32, f32) {
//         let segments: Vec<Segment> = self
//             .points
//             .windows(2)
//             .map(|[x, y]| Segment { start: *x, end: *y })
//             .collect();

//         let points: Vec<(f32, f32)> = segments.iter().map(|s| s.calculate(t)).collect();
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_bezier_second() {
        let b = BezierSecond::new((0.0, 0.0).into(), (2.0, 0.0).into(), (1.0, 1.0).into());

        let t = T::start();
        let res = b.evaluate(t);

        assert_relative_eq!(res.x, 0.0);
        assert_relative_eq!(res.y, 0.0);

        let t = T::end();
        let res = b.evaluate(t);

        assert_relative_eq!(res.x, 2.0);
        assert_relative_eq!(res.y, 0.0);

        let t = T::new(0.5);
        let res = b.evaluate(t);

        assert_relative_eq!(res.x, 1.0);
        assert_relative_eq!(res.y, 0.5);
    }

    #[test]
    fn test_bezier_second_spline() {
        let b = BezierSecondSpline::new(
            vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0), (3.0, 1.0), (4.0, 0.0)]
                .into_iter()
                .map(|p| p.into())
                .collect(),
        );

        let t = T::start();
        let res = b.evaluate(t);

        assert_relative_eq!(res.x, 0.0);
        assert_relative_eq!(res.y, 0.0);

        let t = T::end();
        let res = b.evaluate(t);

        assert_relative_eq!(res.x, 4.0);
        assert_relative_eq!(res.y, 0.0);

        let t = T::new(0.5);
        let res = b.evaluate(t);

        assert_relative_eq!(res.x, 2.0);
        assert_relative_eq!(res.y, 0.0);

        let t = T::new(0.75);
        let res = b.evaluate(t);

        assert_relative_eq!(res.x, 3.0);
        assert_relative_eq!(res.y, 0.5);
    }

    #[test]
    fn test_bezier_third() {
        let b = BezierThird::new(
            (0.0, 0.0).into(),
            (1.0, 0.0).into(),
            (0.0, 1.0).into(),
            (1.0, 1.0).into(),
        );

        let t = T::start();
        let res = b.evaluate(t);

        assert_relative_eq!(res.x, 0.0);
        assert_relative_eq!(res.y, 0.0);

        let t = T::end();
        let res = b.evaluate(t);

        assert_relative_eq!(res.x, 1.0);
        assert_relative_eq!(res.y, 0.0);

        let t = T::new(0.5);
        let res = b.evaluate(t);

        assert_relative_eq!(res.x, 0.5);
        assert_relative_eq!(res.y, 0.75);
    }

    #[test]
    fn test_bezier_third_spline() {
        let b = BezierThirdSpline::new(
            vec![
                (0.0, 0.0),
                (0.0, 1.0),
                (1.0, 1.0),
                (1.0, 0.0),
                (1.0, 1.0),
                (2.0, 1.0),
                (2.0, 0.0),
            ]
            .into_iter()
            .map(|p| p.into())
            .collect(),
        );

        let t = T::start();
        let res = b.evaluate(t);

        assert_relative_eq!(res.x, 0.0);
        assert_relative_eq!(res.y, 0.0);

        let t = T::end();
        let res = b.evaluate(t);

        assert_relative_eq!(res.x, 2.0);
        assert_relative_eq!(res.y, 0.0);

        let t = T::new(0.5);
        let res = b.evaluate(t);

        assert_relative_eq!(res.x, 1.0);
        assert_relative_eq!(res.y, 0.0);

        let t = T::new(0.75);
        let res = b.evaluate(t);

        assert_relative_eq!(res.x, 1.5);
        assert_relative_eq!(res.y, 0.75);
    }

    #[test]
    fn test_bezier_fourth() {
        let b = BezierFourth::new(
            (0.0, 0.0).into(),
            (2.0, 0.0).into(),
            (0.5, 1.0).into(),
            (1.0, 0.5).into(),
            (1.5, 1.0).into(),
        );

        let t = T::start();
        let res = b.evaluate(t);

        assert_relative_eq!(res.x, 0.0);
        assert_relative_eq!(res.y, 0.0);

        let t = T::end();
        let res = b.evaluate(t);

        assert_relative_eq!(res.x, 2.0);
        assert_relative_eq!(res.y, 0.0);

        let t = T::new(0.5);
        let res = b.evaluate(t);

        assert_relative_eq!(res.x, 1.0);
        assert_relative_eq!(res.y, 0.6875);
    }

    #[test]
    fn test_bezier_fourth_spline() {
        let b = BezierFourthSpline::new(
            vec![
                (0.0, 0.0),
                (0.5, 1.0),
                (1.0, 0.5),
                (1.5, 1.0),
                (2.0, 0.0),
                (2.5, 1.0),
                (3.0, 0.5),
                (3.5, 1.0),
                (4.0, 0.0),
            ]
            .into_iter()
            .map(|p| p.into())
            .collect(),
        );

        let t = T::start();
        let res = b.evaluate(t);

        assert_relative_eq!(res.x, 0.0);
        assert_relative_eq!(res.y, 0.0);

        let t = T::end();
        let res = b.evaluate(t);

        assert_relative_eq!(res.x, 4.0);
        assert_relative_eq!(res.y, 0.0);

        let t = T::new(0.5);
        let res = b.evaluate(t);

        assert_relative_eq!(res.x, 2.0);
        assert_relative_eq!(res.y, 0.0);

        let t = T::new(0.75);
        let res = b.evaluate(t);

        assert_relative_eq!(res.x, 3.0);
        assert_relative_eq!(res.y, 0.6875);
    }
}
