use crate::matrices;
use crate::shapes;
use crate::tuples;

#[derive(Debug, Clone, Copy)]
pub enum PatternType {
    Stripe,
    PatternTest,
    Gradient,
    Ring,
    Checkers,
}

#[derive(Debug, Copy, Clone)]
pub struct Pattern {
    pub a: tuples::Color,
    pub b: tuples::Color,
    pub transform: matrices::Matrix4,
    pub pattern_type: PatternType,
}

impl Pattern {
    pub fn stripe_at(&self, p: &tuples::Point) -> tuples::Color {
        let rem = p.x % 2.0;
        if rem < 0.0 {
            if rem.abs() < 1.0 {
                self.b
            } else {
                self.a
            }
        } else {
            if rem.abs() <= 1.0 {
                self.a
            } else {
                self.b
            }
        }
    }

    pub fn gradient_pattern_at(&self, p: &tuples::Point) -> tuples::Color {
        let distance = self.b.subtract(&self.a);
        let fraction = p.x - p.x.trunc();
        let d_times_f = distance.scalar_multiply(&fraction);
        self.a.add(&d_times_f)
    }

    pub fn ring_pattern_at(&self, p: &tuples::Point) -> tuples::Color {
        let rem = ((p.x * p.x) + (p.z * p.z)).sqrt().floor() % 2.0;
        if rem == 0.0 {
            self.a
        } else {
            self.b
        }
    }

    pub fn checkers_pattern_at(&self, p: &tuples::Point) -> tuples::Color {
        let rem = (p.x.floor() + p.y.floor() + p.z.floor()) % 2.0;
        if rem == 0.0 {
            self.a
        } else {
            self.b
        }
    }

    pub fn test_pattern_at(&self, p: &tuples::Point) -> tuples::Color {
        tuples::color(p.x, p.y, p.z)
    }

    pub fn pattern_at_shape(&self, s: &shapes::Shape, p: &tuples::Point) -> tuples::Color {
        let local_point: tuples::Point =
            matrices::matrix4_tuple_multiply(&matrices::matrix4_inverse(&s.transform), &p);
        let pattern_point: tuples::Point = matrices::matrix4_tuple_multiply(
            &matrices::matrix4_inverse(&self.transform),
            &local_point,
        );
        match self.pattern_type {
            PatternType::Stripe => self.stripe_at(&pattern_point),
            PatternType::PatternTest => self.test_pattern_at(&pattern_point),
            PatternType::Gradient => self.gradient_pattern_at(&pattern_point),
            PatternType::Ring => self.ring_pattern_at(&pattern_point),
            PatternType::Checkers => self.checkers_pattern_at(&pattern_point),
        }
    }
}

pub const PATTERN_DEFAULT: Pattern = Pattern {
    a: tuples::COLOR_WHITE,
    b: tuples::COLOR_BLACK,
    transform: matrices::IDENTITY_MATRIX,
    pattern_type: PatternType::Stripe,
};

pub const PATTERN_PINK: Pattern = Pattern {
    a: tuples::COLOR_WHITE,
    b: tuples::COLOR_PINK,
    transform: matrices::IDENTITY_MATRIX,
    pattern_type: PatternType::Stripe,
};

pub fn stripe_pattern(a: tuples::Color, b: tuples::Color) -> Pattern {
    Pattern {
        a: a,
        b: b,
        transform: matrices::IDENTITY_MATRIX,
        pattern_type: PatternType::Stripe,
    }
}

pub fn gradient_pattern(a: tuples::Color, b: tuples::Color) -> Pattern {
    Pattern {
        a: a,
        b: b,
        transform: matrices::IDENTITY_MATRIX,
        pattern_type: PatternType::Gradient,
    }
}

pub fn ring_pattern(a: tuples::Color, b: tuples::Color) -> Pattern {
    Pattern {
        a: a,
        b: b,
        transform: matrices::IDENTITY_MATRIX,
        pattern_type: PatternType::Ring,
    }
}

pub fn checkers_pattern(a: tuples::Color, b: tuples::Color) -> Pattern {
    Pattern {
        a: a,
        b: b,
        transform: matrices::IDENTITY_MATRIX,
        pattern_type: PatternType::Checkers,
    }
}

pub fn test_pattern() -> Pattern {
    let mut p = PATTERN_DEFAULT;
    p.pattern_type = PatternType::PatternTest;
    p
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spheres;
    use crate::transformations;

    #[test]
    fn test_creating_a_stripe_pattern() {
        //Creating a stripe pattern
        assert_eq!(PATTERN_DEFAULT.a.equals(&tuples::COLOR_WHITE), true);
        assert_eq!(PATTERN_DEFAULT.b.equals(&tuples::COLOR_BLACK), true);
    }

    #[test]
    fn test_a_stripe_pattern_is_constant_in_y() {
        //A stripe pattern is constant in y
        let s1 = PATTERN_DEFAULT.stripe_at(&tuples::point(0.0, 0.0, 0.0));
        let s2 = PATTERN_DEFAULT.stripe_at(&tuples::point(0.0, 1.0, 0.0));
        let s3 = PATTERN_DEFAULT.stripe_at(&tuples::point(0.0, 2.0, 0.0));
        assert_eq!(s1.equals(&tuples::COLOR_WHITE), true);
        assert_eq!(s2.equals(&tuples::COLOR_WHITE), true);
        assert_eq!(s3.equals(&tuples::COLOR_WHITE), true);
    }

    #[test]
    fn test_a_stripe_pattern_is_constant_in_z() {
        //A stripe pattern is constant in z
        let s1 = PATTERN_DEFAULT.stripe_at(&tuples::point(0.0, 0.0, 0.0));
        let s2 = PATTERN_DEFAULT.stripe_at(&tuples::point(0.0, 0.0, 1.0));
        let s3 = PATTERN_DEFAULT.stripe_at(&tuples::point(0.0, 0.0, 2.0));
        assert_eq!(s1.equals(&tuples::COLOR_WHITE), true);
        assert_eq!(s2.equals(&tuples::COLOR_WHITE), true);
        assert_eq!(s3.equals(&tuples::COLOR_WHITE), true);
    }

    #[test]
    fn test_a_stripe_pattern_alternates_in_x() {
        //A stripe pattern alternates in x
        let x_values = [
            -4.0, -3.9, -3.5, -3.1, -3.0, -2.9, -2.5, -2.1, -2.0, -1.9, -1.5, -1.1, -1.0, -0.9,
            -0.5, -0.1, 0.0, 0.1, 0.5, 0.9, 1.0, 1.1, 1.5, 1.9, 2.0, 2.1, 2.5, 2.9, 3.0, 3.1, 3.5,
            3.9, 4.0,
        ];
        let x_bools = [
            true, true, true, true, true, false, false, false, true, true, true, true, true, false,
            false, false, true, true, true, true, true, false, false, false, true, true, true,
            true, true, false, false, false, true,
        ];
        for i in 0..x_values.len() {
            let x = x_values[i];
            let stripe_color = PATTERN_DEFAULT.stripe_at(&tuples::point(x, 0.0, 0.0));
            println!("x:{} mod {} red:{} ", x, x % 2.0, stripe_color.red);
            assert_eq!(stripe_color.equals(&tuples::COLOR_WHITE), x_bools[i]);
        }
    }

    #[test]
    fn test_stripes_with_an_object_transformation() {
        //Stripes with an object transformation
        let mut s = spheres::sphere();
        s.transform = transformations::matrix4_scaling(2.0, 2.0, 2.0);
        let stripe_color = PATTERN_DEFAULT.pattern_at_shape(&s, &tuples::point(1.5, 0.0, 0.0));
        assert_eq!(stripe_color.equals(&tuples::COLOR_WHITE), true);
    }

    #[test]
    fn test_stripes_with_a_pattern_transformation() {
        //Stripes with a pattern transformation
        let s = spheres::sphere();
        let mut p = PATTERN_DEFAULT;
        p.transform = transformations::matrix4_scaling(2.0, 2.0, 2.0);
        let stripe_color = p.pattern_at_shape(&s, &tuples::point(1.5, 0.0, 0.0));
        assert_eq!(stripe_color.equals(&tuples::COLOR_WHITE), true);
    }

    #[test]
    fn test_stripes_with_both_an_object_and_a_pattern_transformation() {
        //Stripes with both an object and a pattern transformation
        let mut s = spheres::sphere();
        s.transform = transformations::matrix4_scaling(2.0, 2.0, 2.0);
        let mut p = PATTERN_DEFAULT;
        p.transform = transformations::matrix4_translation(0.5, 0.0, 0.0);
        let stripe_color = p.pattern_at_shape(&s, &tuples::point(2.5, 0.0, 0.0));
        assert_eq!(stripe_color.equals(&tuples::COLOR_WHITE), true);
    }

    #[test]
    fn test_the_default_pattern_transformation() {
        //The default Pattern trasnformation
        let p = test_pattern();
        assert_eq!(p.transform.equals(&matrices::IDENTITY_MATRIX), true);
    }

    #[test]
    fn test_assigning_a_pattern_transformation() {
        //Assigning a pattern transformation
        let mut p = test_pattern();
        let t = transformations::matrix4_translation(1.0, 2.0, 3.0);
        p.transform = t;
        assert_eq!(p.transform.equals(&t), true);
    }

    #[test]
    fn test_a_pattern_with_an_object_transformation() {
        //A pattern with an object transformation
        let mut s = spheres::sphere();
        s.transform = transformations::matrix4_scaling(2.0, 2.0, 2.0);
        let p = test_pattern();
        let c = p.pattern_at_shape(&s, &tuples::point(2.0, 3.0, 4.0));
        assert_eq!(c.equals(&tuples::color(1.0, 1.5, 2.0)), true);
    }

    #[test]
    fn test_a_pattern_with_a_pattern_transformation() {
        //A pattern with a pattern transformation
        let s = spheres::sphere();
        let mut p = test_pattern();
        p.transform = transformations::matrix4_scaling(2.0, 2.0, 2.0);
        let c = p.pattern_at_shape(&s, &tuples::point(2.0, 3.0, 4.0));
        assert_eq!(c.equals(&tuples::color(1.0, 1.5, 2.0)), true);
    }

    #[test]
    fn test_a_pattern_with_both_an_object_and_a_pattern_transformation() {
        //A pattern with both an object and a pattern transformation
        let mut s = spheres::sphere();
        s.transform = transformations::matrix4_scaling(2.0, 2.0, 2.0);
        let mut p = test_pattern();
        p.transform = transformations::matrix4_translation(0.5, 1.0, 1.5);
        let c = p.pattern_at_shape(&s, &tuples::point(2.5, 3.0, 3.5));
        assert_eq!(c.equals(&tuples::color(0.75, 0.5, 0.25)), true);
    }

    #[test]
    fn test_a_gradient_linearly_interpolates_between_colors() {
        //A gradient linearly interpolates between colors
        let p = gradient_pattern(tuples::COLOR_WHITE, tuples::COLOR_BLACK);
        let c1 = p.gradient_pattern_at(&tuples::point(0.0, 0.0, 0.0));
        let c2 = p.gradient_pattern_at(&tuples::point(0.25, 0.0, 0.0));
        let c3 = p.gradient_pattern_at(&tuples::point(0.5, 0.0, 0.0));
        let c4 = p.gradient_pattern_at(&tuples::point(0.75, 0.0, 0.0));
        assert_eq!(c1.equals(&tuples::COLOR_WHITE), true);
        assert_eq!(c2.equals(&tuples::color(0.75, 0.75, 0.75)), true);
        assert_eq!(c3.equals(&tuples::color(0.5, 0.5, 0.5)), true);
        assert_eq!(c4.equals(&tuples::color(0.25, 0.25, 0.25)), true);
    }

    #[test]
    fn test_a_ring_should_extend_in_both_x_and_z() {
        //A ring should extend in both x and z
        let p = ring_pattern(tuples::COLOR_WHITE, tuples::COLOR_BLACK);
        let c1 = p.ring_pattern_at(&tuples::point(0.0, 0.0, 0.0));
        let c2 = p.ring_pattern_at(&tuples::point(1.0, 0.0, 0.0));
        let c3 = p.ring_pattern_at(&tuples::point(0.0, 0.0, 1.0));
        let c4 = p.ring_pattern_at(&tuples::point(0.708, 0.0, 0.708));
        assert_eq!(c1.equals(&tuples::COLOR_WHITE), true);
        assert_eq!(c2.equals(&tuples::COLOR_BLACK), true);
        assert_eq!(c3.equals(&tuples::COLOR_BLACK), true);
        assert_eq!(c4.equals(&tuples::COLOR_BLACK), true);
    }

    #[test]
    fn test_checkers_should_repeat_in_x() {
        //Checkers should repeat in x
        let p = checkers_pattern(tuples::COLOR_WHITE, tuples::COLOR_BLACK);
        let c1 = p.checkers_pattern_at(&tuples::point(0.0, 0.0, 0.0));
        let c2 = p.checkers_pattern_at(&tuples::point(0.99, 0.0, 0.0));
        let c3 = p.checkers_pattern_at(&tuples::point(1.01, 0.0, 0.0));
        assert_eq!(c1.equals(&tuples::COLOR_WHITE), true);
        assert_eq!(c2.equals(&tuples::COLOR_WHITE), true);
        assert_eq!(c3.equals(&tuples::COLOR_BLACK), true);
    }
}
