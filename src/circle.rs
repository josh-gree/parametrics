//! Circles and Rcs

use crate::core::{ParametricFunction2D, Point, T};

/// A circle of radius `r`, centred at a point - parameterisation starting at a given "angle"
/// measured in "turns" (so `[0,1]`) - where `0` is on the positive x-axis for the unit circle.
pub struct Circle {
    pub centre: Point,
    pub radius: f32,
    pub start_angle: T,
}

impl Circle {
    pub fn new(centre: Point, radius: f32, start_angle: Option<T>) -> Self {
        Self {
            centre,
            radius,
            start_angle: start_angle.unwrap_or(T::start()),
        }
    }
}

/// A circle Rc of radius `r`, centred at a point - parameterisation starting at a given "angle" `start_angle`
/// and ending at `end_angle` - "angles" are "turns" as described in [`Circle`]
pub struct CircleArc {
    pub centre: Point,
    pub radius: f32,
    pub start_angle: T,
    pub end_angle: T,
}

impl CircleArc {
    pub fn new(centre: Point, radius: f32, start_angle: Option<T>, end_angle: Option<T>) -> Self {
        Self {
            centre: centre,
            radius: radius,
            start_angle: start_angle.unwrap_or(T::start()),
            end_angle: end_angle.unwrap_or(T::end()),
        }
    }
}

impl ParametricFunction2D for CircleArc {
    fn evaluate(&self, t: T) -> Point {
        let c = self.centre;
        let r = self.radius;
        let start_angle = self.start_angle;
        let end_angle = self.end_angle;

        let theta = end_angle.value() * t.value() + (1.0 - t.value()) * start_angle.value();
        (
            c.x + r * f32::cos(theta * std::f32::consts::TAU),
            c.y + r * f32::sin(theta * std::f32::consts::TAU),
        )
            .into()
    }
}

impl ParametricFunction2D for Circle {
    fn evaluate(&self, t: T) -> Point {
        let c = self.centre;
        let r = self.radius;
        let start_angle = self.start_angle;
        (
            c.x + r * f32::cos((t.value() + start_angle.value()) * std::f32::consts::TAU),
            c.y + r * f32::sin((t.value() + start_angle.value()) * std::f32::consts::TAU),
        )
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use std::f32;

    #[test]
    fn test_circle() {
        let c = Circle::new((0.0, 0.0).into(), 1.0, None);

        let res = c.evaluate(T::new(0.5));
        assert_relative_eq!(res.x, -1.0, epsilon = f32::EPSILON * 10.0);
        assert_relative_eq!(res.y, 0.0, epsilon = f32::EPSILON * 10.0);

        let c = Circle::new((0.0, 0.0).into(), 1.0, Some(T::new(0.5)));

        let res = c.evaluate(T::new(0.5));
        assert_relative_eq!(res.x, 1.0, epsilon = f32::EPSILON * 10.0);
        assert_relative_eq!(res.y, 0.0, epsilon = f32::EPSILON * 10.0);

        let c = Circle::new((1.0, 1.0).into(), 2.0, None);

        let res = c.evaluate(T::new(0.5));
        assert_relative_eq!(res.x, -1.0, epsilon = f32::EPSILON * 10.0);
        assert_relative_eq!(res.y, 1.0, epsilon = f32::EPSILON * 10.0);
    }

    #[test]
    fn test_circle_arc() {
        let ca = CircleArc::new((0.0, 0.0).into(), 1.0, None, Some(T::new(0.25)));

        let res = ca.evaluate(T::start());
        assert_relative_eq!(res.x, 1.0, epsilon = f32::EPSILON * 10.0);
        assert_relative_eq!(res.y, 0.0, epsilon = f32::EPSILON * 10.0);

        let res = ca.evaluate(T::end());
        assert_relative_eq!(res.x, 0.0, epsilon = f32::EPSILON * 10.0);
        assert_relative_eq!(res.y, 1.0, epsilon = f32::EPSILON * 10.0);
    }
}
