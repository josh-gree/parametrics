//! A crate for working with parametric functions

pub mod bezier;
pub mod circle;
pub mod core;
pub mod segment;

pub use crate::bezier::{
    BezierFourth, BezierFourthSpline, BezierSecond, BezierSecondSpline, BezierThird,
    BezierThirdSpline,
};
pub use crate::circle::Circle;
pub use crate::circle::CircleRc;
pub use crate::core::{Concat, Point, Repeat, Rotate, RotateTranslate, Translate, T};
pub use crate::segment::Segment;
