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

pub fn stripe_at(pat: Pattern, p: tuples::Point) -> tuples::Color {
    let rem = p.x % 2.0;
    if rem < 0.0 {
        if rem.abs() < 1.0 {
            pat.b
        } else {
            pat.a
        }
    } else {
        if rem.abs() <= 1.0 {
            pat.a
        } else {
            pat.b
        }
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

pub fn gradient_pattern_at(pat: Pattern, p: tuples::Point) -> tuples::Color {
    let distance = tuples::colors_subtract(&pat.b, &pat.a);
    let fraction = p.x - p.x.trunc();
    let d_times_f = tuples::colors_scalar_multiply(&distance, fraction);
    tuples::colors_add(&pat.a, &d_times_f)
}

pub fn ring_pattern(a: tuples::Color, b: tuples::Color) -> Pattern {
    Pattern {
        a: a,
        b: b,
        transform: matrices::IDENTITY_MATRIX,
        pattern_type: PatternType::Ring,
    }
}

pub fn ring_pattern_at(pat: Pattern, p: tuples::Point) -> tuples::Color {
    let rem = ((p.x * p.x) + (p.z * p.z)).sqrt().floor() % 2.0;
    if rem == 0.0 {
        pat.a
    } else {
        pat.b
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

pub fn checkers_pattern_at(pat: Pattern, p: tuples::Point) -> tuples::Color {
    let rem = (p.x.floor() + p.y.floor() + p.z.floor()) % 2.0;
    if rem == 0.0 {
        pat.a
    } else {
        pat.b
    }
}

pub fn test_pattern() -> Pattern {
    let mut p = PATTERN_DEFAULT;
    p.pattern_type = PatternType::PatternTest;
    p
}

pub fn test_pattern_at(_pat: Pattern, p: tuples::Point) -> tuples::Color {
    tuples::color(p.x, p.y, p.z)
}

pub fn pattern_at_shape(pat: Pattern, s: shapes::Shape, p: tuples::Point) -> tuples::Color {
    let local_point: tuples::Point =
        matrices::matrix4_tuple_multiply(matrices::matrix4_inverse(s.transform), p);
    let pattern_point: tuples::Point =
        matrices::matrix4_tuple_multiply(matrices::matrix4_inverse(pat.transform), local_point);
    match pat.pattern_type {
        PatternType::Stripe => stripe_at(pat, pattern_point),
        PatternType::PatternTest => test_pattern_at(pat, pattern_point),
        PatternType::Gradient => gradient_pattern_at(pat, pattern_point),
        PatternType::Ring => ring_pattern_at(pat, pattern_point),
        PatternType::Checkers => checkers_pattern_at(pat, pattern_point),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spheres;
    use crate::transformations;

    #[test]
    fn test_creating_a_stripe_pattern() {
        //Creating a stripe pattern
        assert_eq!(
            tuples::get_bool_colors_are_equal(&PATTERN_DEFAULT.a, &tuples::COLOR_WHITE),
            true
        );
        assert_eq!(
            tuples::get_bool_colors_are_equal(&PATTERN_DEFAULT.b, &tuples::COLOR_BLACK),
            true
        );
    }

    #[test]
    fn test_a_stripe_pattern_is_constant_in_y() {
        //A stripe pattern is constant in y
        let s1 = stripe_at(PATTERN_DEFAULT, tuples::point(0.0, 0.0, 0.0));
        let s2 = stripe_at(PATTERN_DEFAULT, tuples::point(0.0, 1.0, 0.0));
        let s3 = stripe_at(PATTERN_DEFAULT, tuples::point(0.0, 2.0, 0.0));
        assert_eq!(
            tuples::get_bool_colors_are_equal(&s1, &tuples::COLOR_WHITE),
            true
        );
        assert_eq!(
            tuples::get_bool_colors_are_equal(&s2, &tuples::COLOR_WHITE),
            true
        );
        assert_eq!(
            tuples::get_bool_colors_are_equal(&s3, &tuples::COLOR_WHITE),
            true
        );
    }

    #[test]
    fn test_a_stripe_pattern_is_constant_in_z() {
        //A stripe pattern is constant in z
        let s1 = stripe_at(PATTERN_DEFAULT, tuples::point(0.0, 0.0, 0.0));
        let s2 = stripe_at(PATTERN_DEFAULT, tuples::point(0.0, 0.0, 1.0));
        let s3 = stripe_at(PATTERN_DEFAULT, tuples::point(0.0, 0.0, 2.0));
        assert_eq!(
            tuples::get_bool_colors_are_equal(&s1, &tuples::COLOR_WHITE),
            true
        );
        assert_eq!(
            tuples::get_bool_colors_are_equal(&s2, &tuples::COLOR_WHITE),
            true
        );
        assert_eq!(
            tuples::get_bool_colors_are_equal(&s3, &tuples::COLOR_WHITE),
            true
        );
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
            let stripe_color = stripe_at(PATTERN_DEFAULT, tuples::point(x, 0.0, 0.0));
            println!("x:{} mod {} red:{} ", x, x % 2.0, stripe_color.red);
            assert_eq!(
                tuples::get_bool_colors_are_equal(&stripe_color, &tuples::COLOR_WHITE),
                x_bools[i]
            );
        }
    }

    #[test]
    fn test_stripes_with_an_object_transformation() {
        //Stripes with an object transformation
        let mut s = spheres::sphere();
        s.transform = transformations::matrix4_scaling(2.0, 2.0, 2.0);
        let stripe_color = pattern_at_shape(PATTERN_DEFAULT, s, tuples::point(1.5, 0.0, 0.0));
        assert_eq!(
            tuples::get_bool_colors_are_equal(&stripe_color, &tuples::COLOR_WHITE),
            true
        );
    }

    #[test]
    fn test_stripes_with_a_pattern_transformation() {
        //Stripes with a pattern transformation
        let s = spheres::sphere();
        let mut p = PATTERN_DEFAULT;
        p.transform = transformations::matrix4_scaling(2.0, 2.0, 2.0);
        let stripe_color = pattern_at_shape(p, s, tuples::point(1.5, 0.0, 0.0));
        assert_eq!(
            tuples::get_bool_colors_are_equal(&stripe_color, &tuples::COLOR_WHITE),
            true
        );
    }

    #[test]
    fn test_stripes_with_both_an_object_and_a_pattern_transformation() {
        //Stripes with both an object and a pattern transformation
        let mut s = spheres::sphere();
        s.transform = transformations::matrix4_scaling(2.0, 2.0, 2.0);
        let mut p = PATTERN_DEFAULT;
        p.transform = transformations::matrix4_translation(0.5, 0.0, 0.0);
        let stripe_color = pattern_at_shape(p, s, tuples::point(2.5, 0.0, 0.0));
        assert_eq!(
            tuples::get_bool_colors_are_equal(&stripe_color, &tuples::COLOR_WHITE),
            true
        );
    }

    #[test]
    fn test_the_default_pattern_transformation() {
        //The default Pattern trasnformation
        let p = test_pattern();
        assert_eq!(
            matrices::get_bool_equal_m4(p.transform, matrices::IDENTITY_MATRIX),
            true
        );
    }

    #[test]
    fn test_assigning_a_pattern_transformation() {
        //Assigning a pattern transformation
        let mut p = test_pattern();
        let t = transformations::matrix4_translation(1.0, 2.0, 3.0);
        p.transform = t;
        assert_eq!(matrices::get_bool_equal_m4(p.transform, t), true);
    }

    #[test]
    fn test_a_pattern_with_an_object_transformation() {
        //A pattern with an object transformation
        let mut s = spheres::sphere();
        s.transform = transformations::matrix4_scaling(2.0, 2.0, 2.0);
        let p = test_pattern();
        let c = pattern_at_shape(p, s, tuples::point(2.0, 3.0, 4.0));
        assert_eq!(
            tuples::get_bool_colors_are_equal(&c, &tuples::color(1.0, 1.5, 2.0)),
            true
        );
    }

    #[test]
    fn test_a_pattern_with_a_pattern_transformation() {
        //A pattern with a pattern transformation
        let s = spheres::sphere();
        let mut p = test_pattern();
        p.transform = transformations::matrix4_scaling(2.0, 2.0, 2.0);
        let c = pattern_at_shape(p, s, tuples::point(2.0, 3.0, 4.0));
        assert_eq!(
            tuples::get_bool_colors_are_equal(&c, &tuples::color(1.0, 1.5, 2.0)),
            true
        );
    }

    #[test]
    fn test_a_pattern_with_both_an_object_and_a_pattern_transformation() {
        //A pattern with both an object and a pattern transformation
        let mut s = spheres::sphere();
        s.transform = transformations::matrix4_scaling(2.0, 2.0, 2.0);
        let mut p = test_pattern();
        p.transform = transformations::matrix4_translation(0.5, 1.0, 1.5);
        let c = pattern_at_shape(p, s, tuples::point(2.5, 3.0, 3.5));
        assert_eq!(
            tuples::get_bool_colors_are_equal(&c, &tuples::color(0.75, 0.5, 0.25)),
            true
        );
    }

    #[test]
    fn test_a_gradient_linearly_interpolates_between_colors() {
        //A gradient linearly interpolates between colors
        let p = gradient_pattern(tuples::COLOR_WHITE, tuples::COLOR_BLACK);
        let c1 = gradient_pattern_at(p, tuples::point(0.0, 0.0, 0.0));
        let c2 = gradient_pattern_at(p, tuples::point(0.25, 0.0, 0.0));
        let c3 = gradient_pattern_at(p, tuples::point(0.5, 0.0, 0.0));
        let c4 = gradient_pattern_at(p, tuples::point(0.75, 0.0, 0.0));
        assert_eq!(
            tuples::get_bool_colors_are_equal(&c1, &tuples::COLOR_WHITE),
            true
        );
        assert_eq!(
            tuples::get_bool_colors_are_equal(&c2, &tuples::color(0.75, 0.75, 0.75)),
            true
        );
        assert_eq!(
            tuples::get_bool_colors_are_equal(&c3, &tuples::color(0.5, 0.5, 0.5)),
            true
        );
        assert_eq!(
            tuples::get_bool_colors_are_equal(&c4, &tuples::color(0.25, 0.25, 0.25)),
            true
        );
    }

    #[test]
    fn test_a_ring_should_extend_in_both_x_and_z() {
        //A ring should extend in both x and z
        let p = ring_pattern(tuples::COLOR_WHITE, tuples::COLOR_BLACK);
        let c1 = ring_pattern_at(p, tuples::point(0.0, 0.0, 0.0));
        let c2 = ring_pattern_at(p, tuples::point(1.0, 0.0, 0.0));
        let c3 = ring_pattern_at(p, tuples::point(0.0, 0.0, 1.0));
        let c4 = ring_pattern_at(p, tuples::point(0.708, 0.0, 0.708));
        assert_eq!(
            tuples::get_bool_colors_are_equal(&c1, &tuples::COLOR_WHITE),
            true
        );
        assert_eq!(
            tuples::get_bool_colors_are_equal(&c2, &tuples::COLOR_BLACK),
            true
        );
        assert_eq!(
            tuples::get_bool_colors_are_equal(&c3, &tuples::COLOR_BLACK),
            true
        );
        assert_eq!(
            tuples::get_bool_colors_are_equal(&c4, &tuples::COLOR_BLACK),
            true
        );
    }

    #[test]
    fn test_checkers_should_repeat_in_x() {
        //Checkers should repeat in x
        let p = checkers_pattern(tuples::COLOR_WHITE, tuples::COLOR_BLACK);
        let c1 = checkers_pattern_at(p, tuples::point(0.0, 0.0, 0.0));
        let c2 = checkers_pattern_at(p, tuples::point(0.99, 0.0, 0.0));
        let c3 = checkers_pattern_at(p, tuples::point(1.01, 0.0, 0.0));
        assert_eq!(
            tuples::get_bool_colors_are_equal(&c1, &tuples::COLOR_WHITE),
            true
        );
        assert_eq!(
            tuples::get_bool_colors_are_equal(&c2, &tuples::COLOR_WHITE),
            true
        );
        assert_eq!(
            tuples::get_bool_colors_are_equal(&c3, &tuples::COLOR_BLACK),
            true
        );
    }
}
