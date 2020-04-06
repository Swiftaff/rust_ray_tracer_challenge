use crate::tuples;

#[derive(Debug, Copy, Clone)]
pub struct Pattern {
    pub a: tuples::Color,
    pub b: tuples::Color,
}

pub const PATTERN_DEFAULT: Pattern = Pattern {
    a: tuples::COLOR_WHITE,
    b: tuples::COLOR_BLACK,
};

pub fn stripe_pattern(a: tuples::Color, b: tuples::Color) -> Pattern {
    Pattern { a: a, b: b }
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
