//! Circles and Rcs

use crate::core::{ParametricFunction2D, Point, T};

/// A circle of radius `r`, centred at a point - parameterisation starting at a given "angle"
/// measured in "turns" (so `[0,1]`) - where `0` is on the positive x-axis for the unit circle.
pub struct Circle {
    pub centre: Point,
    pub radius: f32,
    pub start_angle: T,
}

/// A circle Rc of radius `r`, centred at a point - parameterisation starting at a given "angle" `start_angle`
/// and ending at `end_angle` - "angles" are "turns" as described in [`Circle`]
pub struct CircleArc {
    pub centre: Point,
    pub radius: f32,
    pub start_angle: T,
    pub end_angle: T,
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
        let c = Circle {
            centre: (0.0, 0.0).into(),
            radius: 1.0,
            start_angle: T::start(),
        };

        let res = c.evaluate(T::new(0.5));
        assert_relative_eq!(res.x, -1.0, epsilon = f32::EPSILON * 10.0);
        assert_relative_eq!(res.y, 0.0, epsilon = f32::EPSILON * 10.0);

        let c = Circle {
            centre: (0.0, 0.0).into(),
            radius: 1.0,
            start_angle: T::new(0.5),
        };

        let res = c.evaluate(T::new(0.5));
        assert_relative_eq!(res.x, 1.0, epsilon = f32::EPSILON * 10.0);
        assert_relative_eq!(res.y, 0.0, epsilon = f32::EPSILON * 10.0);

        let c = Circle {
            centre: (1.0, 1.0).into(),
            radius: 2.0,
            start_angle: T::start(),
        };

        let res = c.evaluate(T::new(0.5));
        assert_relative_eq!(res.x, -1.0, epsilon = f32::EPSILON * 10.0);
        assert_relative_eq!(res.y, 1.0, epsilon = f32::EPSILON * 10.0);
    }

    #[test]
    fn test_circle_arc() {
        let ca = CircleArc {
            centre: (0.0, 0.0).into(),
            radius: 1.0,
            start_angle: T::start(),
            end_angle: T::new(0.25),
        };

        let res = ca.evaluate(T::start());
        assert_relative_eq!(res.x, 1.0, epsilon = f32::EPSILON * 10.0);
        assert_relative_eq!(res.y, 0.0, epsilon = f32::EPSILON * 10.0);

        let res = ca.evaluate(T::end());
        assert_relative_eq!(res.x, 0.0, epsilon = f32::EPSILON * 10.0);
        assert_relative_eq!(res.y, 1.0, epsilon = f32::EPSILON * 10.0);
    }
}
