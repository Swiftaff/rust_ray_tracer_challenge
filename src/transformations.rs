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
    let mut newTuple = tuple;
    for i in 0..arr.len() {
        newTuple = matrices::matrix4_tuple_multiply(arr[i], newTuple)
    }
    newTuple
}
