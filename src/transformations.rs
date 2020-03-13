use std::f64::consts::PI;

use crate::matrices;
use crate::tuples;

pub fn matrix4_translation(x: f64, y: f64, z: f64) -> matrices::Matrix4 {
    let mut t = matrices::IDENTITY_MATRIX;
    t[0][3] = x;
    t[1][3] = y;
    t[2][3] = z;
    t
}

pub fn matrix4_scaling(x: f64, y: f64, z: f64) -> matrices::Matrix4 {
    let mut t = matrices::IDENTITY_MATRIX;
    t[0][0] = x;
    t[1][1] = y;
    t[2][2] = z;
    t
}

pub fn transform_chain(arr: Vec<matrices::Matrix4>, tuple: tuples::Tuple) -> tuples::Tuple {
    //applied in order provided in array
    let mut new_tuple = tuple;
    for i in 0..arr.len() {
        new_tuple = matrices::matrix4_tuple_multiply(arr[i], new_tuple)
    }
    new_tuple
}

pub fn matrix4_rotation_x_rad(r: f64) -> matrices::Matrix4 {
    let mut t = matrices::IDENTITY_MATRIX;
    t[1][1] = r.cos();
    t[1][2] = -1.0 * r.sin();
    t[2][1] = r.sin();
    t[2][2] = r.cos();
    t
}

pub fn matrix4_rotation_y_rad(r: f64) -> matrices::Matrix4 {
    let mut t = matrices::IDENTITY_MATRIX;
    t[0][0] = r.cos();
    t[0][2] = r.sin();
    t[2][0] = -1.0 * r.sin();
    t[2][2] = r.cos();
    t
}

pub fn matrix4_rotation_z_rad(r: f64) -> matrices::Matrix4 {
    let mut t = matrices::IDENTITY_MATRIX;
    t[0][0] = r.cos();
    t[0][1] = -1.0 * r.sin();
    t[1][0] = r.sin();
    t[1][1] = r.cos();
    t
}

pub fn matrix4_shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> matrices::Matrix4 {
    let mut t = matrices::IDENTITY_MATRIX;
    t[0][1] = xy;
    t[0][2] = xz;
    t[1][0] = yx;
    t[1][2] = yz;
    t[2][0] = zx;
    t[2][1] = zy;
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiply() {
        //translation
        let p = tuples::point(-3.0, 4.0, 5.0);
        let t = matrix4_translation(5.0, -3.0, 2.0);
        let r = tuples::point(2.0, 1.0, 7.0);
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&matrices::matrix4_tuple_multiply(t, p), &r),
            true
        );
    }

    #[test]
    fn test_matrix_multiply_inverse_translation() {
        //Multiplying by the inverse of a translation matrix
        let p = tuples::point(-3.0, 4.0, 5.0);
        let t = matrix4_translation(5.0, -3.0, 2.0);
        let i = matrices::matrix4_inverse(t);
        let r = tuples::point(-8.0, 7.0, 3.0);
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&matrices::matrix4_tuple_multiply(i, p), &r),
            true
        );
    }

    #[test]
    fn test_translation_not_affect_vectors() {
        //Translation does not affect vectors
        let v = tuples::vector(-3.0, 4.0, 5.0);
        let t = matrix4_translation(5.0, -3.0, 2.0);
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&matrices::matrix4_tuple_multiply(t, v), &v),
            true
        );
    }

    #[test]
    fn test_scaling_point() {
        //A scaling matrix applied to a point
        let p = tuples::point(-4.0, 6.0, 8.0);
        let t = matrix4_scaling(2.0, 3.0, 4.0);
        let r = tuples::point(-8.0, 18.0, 32.0);
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&matrices::matrix4_tuple_multiply(t, p), &r),
            true
        );
    }

    #[test]
    fn test_scaling_vector() {
        //A scaling matrix applied to a vector
        let v = tuples::vector(-4.0, 6.0, 8.0);
        let t = matrix4_scaling(2.0, 3.0, 4.0);
        let r = tuples::vector(-8.0, 18.0, 32.0);
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&matrices::matrix4_tuple_multiply(t, v), &r),
            true
        );
    }

    #[test]
    fn test_inverse_scaling() {
        //Multiplying by the inverse of a scaling matrix
        let v = tuples::vector(-4.0, 6.0, 8.0);
        let t = matrix4_scaling(2.0, 3.0, 4.0);
        let i = matrices::matrix4_inverse(t);
        let iv = matrices::matrix4_tuple_multiply(i, v);
        let r = tuples::vector(-2.0, 2.0, 2.0);
        assert_eq!(tuples::get_bool_tuples_are_equal(&iv, &r), true);
    }

    #[test]
    fn test_reflection() {
        //Reflection is scaling by a negative value
        let p = tuples::point(2.0, 3.0, 4.0);
        let t = matrix4_scaling(-1.0, 1.0, 1.0);
        let r = tuples::point(-2.0, 3.0, 4.0);
        let iv = matrices::matrix4_tuple_multiply(t, p);
        assert_eq!(tuples::get_bool_tuples_are_equal(&iv, &r), true);
    }

    #[test]
    fn test_rotating_point_x_axis() {
        //Rotating a point around the x axis
        let p = tuples::point(0.0, 1.0, 0.0);
        let half_quarter = matrix4_rotation_x_rad(PI / 4.0);
        let full_quarter = matrix4_rotation_x_rad(PI / 2.0);
        let result1 = tuples::point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0);
        let result2 = tuples::point(0.0, 0.0, 1.0);
        assert_eq!(
            tuples::get_bool_tuples_are_equal(
                &matrices::matrix4_tuple_multiply(half_quarter, p),
                &result1
            ),
            true
        );
        assert_eq!(
            tuples::get_bool_tuples_are_equal(
                &matrices::matrix4_tuple_multiply(full_quarter, p),
                &result2
            ),
            true
        );
    }

    #[test]
    fn test_rotating_point_x_axis_opposite() {
        //The inverse of an x-rotation rotates in the opposite direction
        let p = tuples::point(0.0, 1.0, 0.0);
        let half_quarter = matrix4_rotation_x_rad(PI / 4.0);
        let inv = matrices::matrix4_inverse(half_quarter);
        let result = tuples::point(0.0, 2.0_f64.sqrt() / 2.0, -1.0 * 2.0_f64.sqrt() / 2.0);
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&matrices::matrix4_tuple_multiply(inv, p), &result),
            true
        );
    }

    #[test]
    fn test_rotating_point_y_axis() {
        //Rotating a point around the y axis
        let p = tuples::point(0.0, 0.0, 1.0);
        let half_quarter = matrix4_rotation_y_rad(PI / 4.0);
        let full_quarter = matrix4_rotation_y_rad(PI / 2.0);
        let result1 = tuples::point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0);
        let result2 = tuples::point(1.0, 0.0, 0.0);
        assert_eq!(
            tuples::get_bool_tuples_are_equal(
                &matrices::matrix4_tuple_multiply(half_quarter, p),
                &result1
            ),
            true
        );
        assert_eq!(
            tuples::get_bool_tuples_are_equal(
                &matrices::matrix4_tuple_multiply(full_quarter, p),
                &result2
            ),
            true
        );
    }

    #[test]
    fn test_rotating_point_z_axis() {
        //Rotating a point around the z axis
        let p = tuples::point(0.0, 1.0, 0.0);
        let half_quarter = matrix4_rotation_z_rad(PI / 4.0);
        let full_quarter = matrix4_rotation_z_rad(PI / 2.0);
        let result1 = tuples::point(-1.0 * 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);
        let result2 = tuples::point(-1.0, 0.0, 0.0);
        assert_eq!(
            tuples::get_bool_tuples_are_equal(
                &matrices::matrix4_tuple_multiply(half_quarter, p),
                &result1
            ),
            true
        );
        assert_eq!(
            tuples::get_bool_tuples_are_equal(
                &matrices::matrix4_tuple_multiply(full_quarter, p),
                &result2
            ),
            true
        );
    }

    #[test]
    fn test_shearing_x_y() {
        //A shearing transformation moves x in proportion to y
        let p = tuples::point(2.0, 3.0, 4.0);
        let t = matrix4_shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let r = tuples::point(5.0, 3.0, 4.0);
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&matrices::matrix4_tuple_multiply(t, p), &r),
            true
        );
    }

    #[test]
    fn test_shearing_x_z() {
        //A shearing transformation moves x in proportion to z
        let p = tuples::point(2.0, 3.0, 4.0);
        let t = matrix4_shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let r = tuples::point(6.0, 3.0, 4.0);
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&matrices::matrix4_tuple_multiply(t, p), &r),
            true
        );
    }

    #[test]
    fn test_shearing_y_x() {
        //A shearing transformation moves y in proportion to x
        let p = tuples::point(2.0, 3.0, 4.0);
        let t = matrix4_shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let r = tuples::point(2.0, 5.0, 4.0);
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&matrices::matrix4_tuple_multiply(t, p), &r),
            true
        );
    }

    #[test]
    fn test_shearing_y_z() {
        //A shearing transformation moves y in proportion to z
        let p = tuples::point(2.0, 3.0, 4.0);
        let t = matrix4_shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let r = tuples::point(2.0, 7.0, 4.0);
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&matrices::matrix4_tuple_multiply(t, p), &r),
            true
        );
    }

    #[test]
    fn test_shearing_z_x() {
        //A shearing transformation moves z in proportion to x
        let p = tuples::point(2.0, 3.0, 4.0);
        let t = matrix4_shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let r = tuples::point(2.0, 3.0, 6.0);
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&matrices::matrix4_tuple_multiply(t, p), &r),
            true
        );
    }

    #[test]
    fn test_shearing_z_y() {
        //A shearing transformation moves z in proportion to y
        let p = tuples::point(2.0, 3.0, 4.0);
        let t = matrix4_shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let r = tuples::point(2.0, 3.0, 7.0);
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&matrices::matrix4_tuple_multiply(t, p), &r),
            true
        );
    }

    #[test]
    fn test_transformations() {
        //Individual transformations are applied in sequence
        let p = tuples::point(1.0, 0.0, 1.0);
        let a = matrix4_rotation_x_rad(PI / 2.0);
        let b = matrix4_scaling(5.0, 5.0, 5.0);
        let c = matrix4_translation(10.0, 5.0, 7.0);
        let p2 = matrices::matrix4_tuple_multiply(a, p);
        let p3 = matrices::matrix4_tuple_multiply(b, p2);
        let p4 = matrices::matrix4_tuple_multiply(c, p3);
        let p5 = transform_chain([a, b, c].to_vec(), p);
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&p2, &tuples::point(1.0, -1.0, 0.0)),
            true
        );
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&p3, &tuples::point(5.0, -5.0, 0.0)),
            true
        );
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&p4, &tuples::point(15.0, 0.0, 7.0)),
            true
        );
        assert_eq!(tuples::get_bool_tuples_are_equal(&p4, &p5), true);
    }
}
