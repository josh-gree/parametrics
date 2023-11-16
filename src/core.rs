//! Core structs and traits

use std::rc::Rc;

use euclid::{Point2D, UnknownUnit};
use rand::prelude::*;

/// The parametric value t
#[derive(Clone, Copy, PartialEq)]
pub struct T(f32);

impl T {
    /// values outside 0 to 1 will be clamped!
    pub fn new(value: f32) -> Self {
        if value <= 0.0 {
            return T(0.0);
        }

        if value >= 1.0 {
            return T(1.0);
        }

        T(value)
    }

    /// returns the value of the `[T]`
    pub fn value(&self) -> f32 {
        self.0
    }

    /// returns "Zero"
    pub fn start() -> Self {
        Self(0.0)
    }

    /// returns "One"
    pub fn end() -> Self {
        Self(1.0)
    }
}

/// Point type from Euclid
pub type Point = Point2D<f32, UnknownUnit>;

/// 2D parametric function trait
pub trait ParametricFunction2D {
    /// returns the value of the parametric function at the point `t`
    fn evaluate(&self, t: T) -> Point;

    /// returns `n` equally spaced points along the entire parametric function from [`T::start`] to [`T::end`]
    fn linspace(&self, n: usize) -> Vec<Point> {
        let step_size = 1.0 / n as f32;
        (0..=n)
            .map(|i| {
                let t = T::new((i as f32) * step_size);
                self.evaluate(t)
            })
            .collect()
    }

    /// returns start, or "first", point on the parametric function
    fn start(&self) -> Point {
        self.evaluate(T::start())
    }

    /// returns end, or"last", point on the parametric function
    fn end(&self) -> Point {
        self.evaluate(T::end())
    }

    /// return a random point on the parametric function
    fn random_point(&self) -> Point {
        let mut rng = rand::thread_rng();
        let t = T::new(rng.gen());
        self.evaluate(t)
    }

    /// return n random points on the parametric function
    fn random_points(&self, n: usize) -> Vec<Point> {
        (0..n).map(|_| self.random_point()).collect()
    }
}

/// 1D parametric function trait
pub trait ParametricFunction1D {
    /// returns the value of the parametric function at the point `t`
    fn evaluate(&self, t: T) -> f32;

    /// returns `n` equally spaced points along the entire parametric function from [`T::start`] to [`T::end`]
    fn linspace(&self, n: usize) -> Vec<f32> {
        let step_size = 1.0 / n as f32;
        (0..=n)
            .map(|i| {
                let t = T::new((i as f32) * step_size);
                self.evaluate(t)
            })
            .collect()
    }

    /// returns start, or "first", point on the parametric function
    fn start(&self) -> f32 {
        self.evaluate(T::start())
    }

    /// returns end, or"last", point on the parametric function
    fn end(&self) -> f32 {
        self.evaluate(T::end())
    }

    /// return a random point on the parametric function
    fn random_point(&self) -> f32 {
        let mut rng = rand::thread_rng();
        let t = T::new(rng.gen());
        self.evaluate(t)
    }

    /// return n random points on the parametric function
    fn random_points(&self, n: usize) -> Vec<f32> {
        (0..n).map(|_| self.random_point()).collect()
    }
}

/// The concatenation of multiple things that implement [`ParametricFunction2D`]
pub struct Concat {
    pub functions: Vec<Rc<Box<dyn ParametricFunction2D>>>,
}

impl ParametricFunction2D for Concat {
    fn evaluate(&self, t: T) -> Point {
        if t == T::start() {
            return self.functions[0].evaluate(t);
        }

        if t == T::end() {
            return self.functions[self.functions.len() - 1].evaluate(t);
        }

        let gap = 1.0 / self.functions.len() as f32;
        let interp = self.functions.len() as f32 * t.value();
        let index = interp.floor() as usize;

        let diff = t.value() - (index as f32) * gap;

        let interp_t = T::new(diff / gap);

        self.functions[index].evaluate(interp_t)
    }
}

/// The repetition `n` times of a thing that implements [`ParametricFunction2D`]
pub struct Repeat {
    pub function: Rc<Box<dyn ParametricFunction2D>>,
    pub n: usize,
}
impl ParametricFunction2D for Repeat {
    fn evaluate(&self, t: T) -> Point {
        let functions = (0..self.n).map(|_| self.function.clone()).collect();
        let concat = Concat { functions };
        concat.evaluate(t)
    }
}
/// The rotation around `centre` by `angle` (in "turns") of a thing that implements [`ParametricFunction2D`]
pub struct Rotate {
    pub function: Rc<Box<dyn ParametricFunction2D>>,
    pub centre: Point,
    pub angle: T,
}
impl ParametricFunction2D for Rotate {
    fn evaluate(&self, t: T) -> Point {
        let val = self.function.evaluate(t);

        (
            self.centre.x
                + (val.x - self.centre.x) * f32::cos(self.angle.value() * std::f32::consts::TAU)
                - (val.y - self.centre.y) * f32::sin(self.angle.value() * std::f32::consts::TAU),
            self.centre.y
                + (val.x - self.centre.x) * f32::sin(self.angle.value() * std::f32::consts::TAU)
                + (val.y - self.centre.y) * f32::cos(self.angle.value() * std::f32::consts::TAU),
        )
            .into()
    }
}

/// The translation by `by` of a thing that implements [`ParametricFunction2D`]
pub struct Translate {
    pub function: Rc<Box<dyn ParametricFunction2D>>,
    pub by: Point,
}

impl ParametricFunction2D for Translate {
    fn evaluate(&self, t: T) -> Point {
        let val = self.function.evaluate(t);
        (val.x + self.by.x, val.y + self.by.y).into()
    }
}

/// Combination of [`Rotate`] and [`Translate`]
pub struct RotateTranslate {
    pub function: Rc<Box<dyn ParametricFunction2D>>,
    pub by: Point,
    pub centre: Point,
    pub angle: T,
    pub rotate_first: bool,
}

impl ParametricFunction2D for RotateTranslate {
    fn evaluate(&self, t: T) -> Point {
        if self.rotate_first {
            let r = Rotate {
                function: self.function.clone(),
                centre: self.centre,
                angle: self.angle,
            };
            let tr = Translate {
                function: Rc::new(Box::new(r)),
                by: self.by,
            };
            tr.evaluate(t)
        } else {
            let tr = Translate {
                function: self.function.clone(),
                by: self.by,
            };
            let r = Rotate {
                function: Rc::new(Box::new(tr)),
                centre: self.centre,
                angle: self.angle,
            };
            r.evaluate(t)
        }
    }
}

impl<F> ParametricFunction2D for F
where
    F: Fn(T) -> Point,
{
    fn evaluate(&self, t: T) -> Point {
        self(t)
    }
}

impl<F> ParametricFunction1D for F
where
    F: Fn(T) -> f32,
{
    fn evaluate(&self, t: T) -> f32 {
        self(t)
    }
}

impl<F, G> ParametricFunction2D for (F, G)
where
    F: ParametricFunction1D,
    G: ParametricFunction1D,
{
    fn evaluate(&self, t: T) -> Point {
        (self.0.evaluate(t), self.1.evaluate(t)).into()
    }
}

pub struct Scale {
    pub function: Rc<Box<dyn ParametricFunction2D>>,
    pub centre: Point,
    pub scale_x: f32,
    pub scale_y: f32,
}

impl ParametricFunction2D for Scale {
    fn evaluate(&self, t: T) -> Point {
        let val = self.function.evaluate(t);
        let val_trans_origin: Point = (val.x - self.centre.x, val.y - self.centre.y).into();
        let scaled: Point = (
            val_trans_origin.x * self.scale_x,
            val_trans_origin.y * self.scale_y,
        )
            .into();
        (scaled.x + self.centre.x, scaled.y + self.centre.y).into()
    }
}
#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use crate::{segment::Segment, Circle};

    use super::*;

    #[test]
    fn test_repeat() {
        let s = Segment {
            start: (0.0, 0.0).into(),
            end: (1.0, 1.0).into(),
        };
        let rep = Repeat {
            function: Rc::new(Box::new(s)),
            n: 2,
        };

        let res = rep.evaluate(T::start());

        assert_relative_eq!(res.x, 0.0);
        assert_relative_eq!(res.y, 0.0);

        let res = rep.evaluate(T::end());

        assert_relative_eq!(res.x, 1.0);
        assert_relative_eq!(res.y, 1.0);

        let res = rep.evaluate(T::new(0.5));

        assert_relative_eq!(res.x, 0.0);
        assert_relative_eq!(res.y, 0.0);
    }

    #[test]
    fn test_concat() {
        let s1 = Segment {
            start: (0.0, 0.0).into(),
            end: (1.0, 1.0).into(),
        };
        let s2 = Segment {
            start: (1.0, 1.0).into(),
            end: (0.0, 2.0).into(),
        };

        let concat = Concat {
            functions: vec![Rc::new(Box::new(s1)), Rc::new(Box::new(s2))],
        };

        let res = concat.evaluate(T::start());

        assert_relative_eq!(res.x, 0.0);
        assert_relative_eq!(res.y, 0.0);

        let res = concat.evaluate(T::end());

        assert_relative_eq!(res.x, 0.0);
        assert_relative_eq!(res.y, 2.0);

        let res = concat.evaluate(T::new(0.5));

        assert_relative_eq!(res.x, 1.0);
        assert_relative_eq!(res.y, 1.0);
    }

    #[test]
    fn test_concat_repeat() {
        let s1 = Segment {
            start: (0.0, 0.0).into(),
            end: (1.0, 1.0).into(),
        };
        let s2 = Segment {
            start: (1.0, 1.0).into(),
            end: (0.0, 2.0).into(),
        };

        let concat = Concat {
            functions: vec![Rc::new(Box::new(s1)), Rc::new(Box::new(s2))],
        };
        let repeat = Repeat {
            function: Rc::new(Box::new(concat)),
            n: 2,
        };

        let res = repeat.evaluate(T::start());
        assert_relative_eq!(res.x, 0.0);
        assert_relative_eq!(res.y, 0.0);

        let res = repeat.evaluate(T::end());
        assert_relative_eq!(res.x, 0.0);
        assert_relative_eq!(res.y, 2.0);

        let res = repeat.evaluate(T::new(0.5));
        assert_relative_eq!(res.x, 0.0);
        assert_relative_eq!(res.y, 0.0);

        let res = repeat.evaluate(T::new(0.75));
        assert_relative_eq!(res.x, 1.0);
        assert_relative_eq!(res.y, 1.0);

        let res = repeat.evaluate(T::new(0.125));
        assert_relative_eq!(res.x, 0.5);
        assert_relative_eq!(res.y, 0.5);
    }

    #[test]
    fn test_random() {
        let s = Segment {
            start: (0.0, 0.0).into(),
            end: (1.0, 1.0).into(),
        };

        let _p = s.random_point();
        let ps = s.random_points(100);
        assert_eq!(ps.len(), 100)
    }

    #[test]
    fn test_rotate() {
        let s = Segment {
            start: (0.0, 0.0).into(),
            end: (1.0, 1.0).into(),
        };
        let r = Rotate {
            function: Rc::new(Box::new(s)),
            centre: (0.5, 0.5).into(),
            angle: T::new(0.25),
        };

        let t = T::start();
        let res = r.evaluate(t);

        assert_relative_eq!(res.x, 1.0, epsilon = f32::EPSILON * 10.0);
        assert_relative_eq!(res.y, 0.0, epsilon = f32::EPSILON * 10.0);

        let t = T::end();
        let res = r.evaluate(t);

        assert_relative_eq!(res.x, 0.0, epsilon = f32::EPSILON * 10.0);
        assert_relative_eq!(res.y, 1.0, epsilon = f32::EPSILON * 10.0);
    }

    #[test]
    fn test_translate() {
        let s = Segment {
            start: (0.0, 0.0).into(),
            end: (1.0, 1.0).into(),
        };
        let tr = Translate {
            function: Rc::new(Box::new(s)),
            by: (0.5, 0.5).into(),
        };

        let t = T::start();
        let res = tr.evaluate(t);

        assert_relative_eq!(res.x, 0.5, epsilon = f32::EPSILON * 10.0);
        assert_relative_eq!(res.y, 0.5, epsilon = f32::EPSILON * 10.0);

        let t = T::end();
        let res = tr.evaluate(t);

        assert_relative_eq!(res.x, 1.5, epsilon = f32::EPSILON * 10.0);
        assert_relative_eq!(res.y, 1.5, epsilon = f32::EPSILON * 10.0);
    }

    #[test]
    fn test_rotate_translate() {
        let s = Segment {
            start: (0.0, 0.0).into(),
            end: (1.0, 1.0).into(),
        };
        let r_tr = RotateTranslate {
            function: Rc::new(Box::new(s)),
            centre: (0.5, 0.5).into(),
            angle: T::new(0.25),
            by: (0.5, 0.5).into(),
            rotate_first: true,
        };

        let t = T::start();
        let res = r_tr.evaluate(t);

        assert_relative_eq!(res.x, 1.5, epsilon = f32::EPSILON * 10.0);
        assert_relative_eq!(res.y, 0.5, epsilon = f32::EPSILON * 10.0);

        let t = T::end();
        let res = r_tr.evaluate(t);

        assert_relative_eq!(res.x, 0.5, epsilon = f32::EPSILON * 10.0);
        assert_relative_eq!(res.y, 1.5, epsilon = f32::EPSILON * 10.0);

        let s = Segment {
            start: (0.0, 0.0).into(),
            end: (1.0, 1.0).into(),
        };
        let r_tr = RotateTranslate {
            function: Rc::new(Box::new(s)),
            centre: (0.5, 0.5).into(),
            angle: T::new(0.25),
            by: (0.5, 0.5).into(),
            rotate_first: false,
        };

        let t = T::start();
        let res = r_tr.evaluate(t);

        assert_relative_eq!(res.x, 0.5, epsilon = f32::EPSILON * 10.0);
        assert_relative_eq!(res.y, 0.5, epsilon = f32::EPSILON * 10.0);

        let t = T::end();
        let res = r_tr.evaluate(t);

        assert_relative_eq!(res.x, -0.5, epsilon = f32::EPSILON * 10.0);
        assert_relative_eq!(res.y, 1.5, epsilon = f32::EPSILON * 10.0);
    }

    #[test]
    fn test_for_closures() {
        let foo = |t: T| Into::<Point>::into((t.value(), t.value()));

        let res = foo.evaluate(T::start());
        assert_relative_eq!(res.x, 0.0);
        assert_relative_eq!(res.y, 0.0);

        let c = Repeat {
            function: Rc::new(Box::new(foo)),
            n: 2,
        };
        c.linspace(10);
    }

    #[test]
    fn test_1d() {
        let foo = |t: T| t.value();
        let res = foo.evaluate(T::start());
        assert_relative_eq!(res, 0.0);

        let bar = (foo, foo);
        let res = bar.evaluate(T::start());
        assert_relative_eq!(res.x, 0.0);
        assert_relative_eq!(res.y, 0.0);

        let rep = Repeat {
            function: Rc::new(Box::new(bar)),
            n: 2,
        };

        let res = rep.evaluate(T::new(0.5));
        assert_relative_eq!(res.x, 0.0);
        assert_relative_eq!(res.y, 0.0);
    }

    #[test]
    fn test_scale() {
        let c = Circle::new((1.0, 1.0).into(), 10.0, None);
        let scaled_c = Scale {
            function: Rc::new(Box::new(c)),
            centre: (1.0, 1.0).into(),
            scale_x: 0.5,
            scale_y: 2.0,
        };

        let s = scaled_c.start();
        assert_relative_eq!(s.x, 6.0, epsilon = f32::EPSILON * 10.0);
        assert_relative_eq!(s.y, 1.0, epsilon = f32::EPSILON * 10.0);

        let s = scaled_c.evaluate(T::new(0.25));
        assert_relative_eq!(s.x, 1.0, epsilon = f32::EPSILON * 10.0);
        assert_relative_eq!(s.y, 21.0, epsilon = f32::EPSILON * 10.0);
    }
}
