# Parametrics

A crate the allows for working with parametric functions.

## 2D Parametric functions

Contains a trait for 2d parametric functions and the following implementations;

- [Segment](https://docs.rs/parametrics/newest/parametrics/segment/struct.Segment.html)
- [Circle](https://docs.rs/parametrics/newest/parametrics/circle/struct.Circle.html)
- [CircleArc](https://docs.rs/parametrics/newest/parametrics/circle/struct.CircleArc.html)
- [Bezier{Second,Third,Fourth}](https://docs.rs/parametrics/newest/parametrics/bezier/)
- [Bezier{Second,Third,Fourth}Spline](https://docs.rs/parametrics/newest/parametrics/bezier/)

Also allows for combination and modification of things which implement the trait.

- [Repeat](https://docs.rs/parametrics/newest/parametrics/core/struct.Repeat.html)
- [Concat](https://docs.rs/parametrics/newest/parametrics/core/struct.Concat.html)
- [Rotate](https://docs.rs/parametrics/newest/parametrics/core/struct.Rotate.html)
- [Translate](https://docs.rs/parametrics/newest/parametrics/core/struct.Translate.html)
- [RotateTranslate](https://docs.rs/parametrics/newest/parametrics/core/struct.RotateTranslate.html)

The trait is implemented for `Fn(T) -> Point` and `(F,G) where F: ParametricFunction1D, G: ParametricFunction1D`

## 1D Parametric functions

The trait is implemented for `Fn(T) -> f32`


