use crate::tuples;

const IDENTITY_MATRIX: Matrix4 = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

type Matrix2 = [[f64; 2]; 2];
type Matrix3 = [[f64; 3]; 3];
type Matrix4 = [[f64; 4]; 4];

pub fn create_matrix4() -> Matrix4 {
    let row: [f64; 4] = [0.0; 4];
    let arr: [[f64; 4]; 4] = [row; 4];
    arr
}

pub fn create_matrix3() -> Matrix3 {
    let row: [f64; 3] = [0.0; 3];
    let arr: [[f64; 3]; 3] = [row; 3];
    arr
}

pub fn create_matrix2() -> Matrix2 {
    let row: [f64; 2] = [0.0; 2];
    let arr: [[f64; 2]; 2] = [row; 2];
    arr
}

pub fn getM2(m: Matrix2, y: usize, x: usize) -> f64 {
    m[y][x]
}

pub fn getM3(m: Matrix3, y: usize, x: usize) -> f64 {
    m[y][x]
}
pub fn getM4(m: Matrix4, y: usize, x: usize) -> f64 {
    m[y][x]
}

pub fn get_bool_equal_m4(m1: Matrix4, m2: Matrix4) -> bool {
    let mut areEqual = true;
    let rows = m1.len();
    let cols = m1[0].len();
    for y in 0..rows {
        for x in 0..cols {
            if tuples::get_bool_numbers_are_equal(m1[y][x], m2[y][x]) == false {
                areEqual = false;
            }
        }
    }
    areEqual
}

pub fn get_bool_equal_m3(m1: Matrix3, m2: Matrix3) -> bool {
    let mut areEqual = true;
    let rows = m1.len();
    let cols = m1[0].len();
    for y in 0..rows {
        for x in 0..cols {
            if tuples::get_bool_numbers_are_equal(m1[y][x], m2[y][x]) == false {
                areEqual = false;
            }
        }
    }
    areEqual
}

pub fn get_bool_equal_m2(m1: Matrix2, m2: Matrix2) -> bool {
    let mut areEqual = true;
    let rows = m1.len();
    let cols = m1[0].len();
    for y in 0..rows {
        for x in 0..cols {
            if tuples::get_bool_numbers_are_equal(m1[y][x], m2[y][x]) == false {
                areEqual = false;
            }
        }
    }
    areEqual
}

pub fn matrix4_multiply(m1: Matrix4, m2: Matrix4) -> Matrix4 {
    let mut result = create_matrix4();
    for y in 0..4 {
        for x in 0..4 {
            let mut thisResult = 0.0;
            for xx in 0..4 {
                thisResult = thisResult + m1[y][xx] * m2[xx][x];
            }
            result[y][x] = thisResult; //trunc(thisResult);
        }
    }
    result
}

pub fn matrix4_transpose(m: Matrix4) -> Matrix4 {
    let mut result = create_matrix4();
    for y in 0..4 {
        for x in 0..4 {
            result[y][x] = m[x][y];
        }
    }
    result
}

pub fn matrix2_determinant(m: Matrix2) -> f64 {
    m[0][0] * m[1][1] - m[0][1] * m[1][0]
}

pub fn matrix4_tuple_multiply(m1: Matrix4, t: tuples::Tuple) -> tuples::Tuple {
    let mut result = [[0.0], [0.0], [0.0], [0.0]];
    let m2 = [[t.x], [t.y], [t.z], [t.w as f64]];
    for y in 0..4 {
        for x in 0..1 {
            let mut thisResult = 0.0;
            for xx in 0..4 {
                thisResult = thisResult + m1[y][xx] * m2[xx][x];
            }
            result[y][x] = thisResult;
        }
    }
    tuples::Tuple {
        x: result[0][0],
        y: result[1][0],
        z: result[2][0],
        w: result[3][0] as u32,
    }
}

pub fn matrix3_multiply(m1: Matrix3, m2: Matrix3) -> Matrix3 {
    let mut result = create_matrix3();
    for y in 0..3 {
        for x in 0..3 {
            let mut thisResult = 0.0;
            for xx in 0..3 {
                thisResult = thisResult + m1[y][xx] * m2[xx][x];
            }
            result[y][x] = thisResult;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix4() {
        //Creating a matrix
        let m = [
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ];
        assert_eq!(getM4(m, 0, 0), 1.0);
        assert_eq!(getM4(m, 0, 3), 4.0);
        assert_eq!(getM4(m, 1, 0), 5.5);
        assert_eq!(getM4(m, 1, 2), 7.5);
        assert_eq!(getM4(m, 2, 2), 11.0);
        assert_eq!(getM4(m, 3, 0), 13.5);
        assert_eq!(getM4(m, 3, 2), 15.5);
    }

    #[test]
    fn test_matrix3() {
        //A 3x3 matrix ought to be representable
        let m = [[1.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]];
        assert_eq!(getM3(m, 0, 0), 1.0);
        assert_eq!(getM3(m, 0, 2), 3.0);
        assert_eq!(getM3(m, 1, 0), 5.5);
        assert_eq!(getM3(m, 1, 2), 7.5);
        assert_eq!(getM3(m, 2, 2), 11.0);
        assert_eq!(getM3(m, 2, 0), 9.0);
    }

    #[test]
    fn test_matrix2() {
        //A 2x2 matrix ought to be representable
        let m = [[-3.0, 5.0], [1.0, -2.0]];
        assert_eq!(getM2(m, 0, 0), -3.0);
        assert_eq!(getM2(m, 0, 1), 5.0);
        assert_eq!(getM2(m, 1, 0), 1.0);
        assert_eq!(getM2(m, 1, 1), -2.0);
    }

    #[test]
    fn test_matrices_are_equal_m4() {
        //Matrix equality with identical matrices4
        let m1 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ];
        let m2 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ];
        assert_eq!(get_bool_equal_m4(m1, m2), true);
    }

    #[test]
    fn test_matrices_are_equal_m3() {
        //Matrix equality with identical matrices3
        let m1 = [[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 8.0, 7.0]];
        let m2 = [[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 8.0, 7.0]];
        assert_eq!(get_bool_equal_m3(m1, m2), true);
    }

    #[test]
    fn test_matrices_are_equal_m2() {
        //Matrix equality with identical matrices2
        let m1 = [[1.0, 2.0], [5.0, 6.0]];
        let m2 = [[1.0, 2.0], [5.0, 6.0]];
        assert_eq!(get_bool_equal_m2(m1, m2), true);
    }

    #[test]
    fn test_matrices_are_not_equal_m4() {
        //Matrix equality with different matrices4
        let m1 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ];
        let m2 = [
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
            [1.0, 2.0, 3.0, 4.0],
        ];
        assert_eq!(get_bool_equal_m4(m1, m2), false);
    }

    #[test]
    fn test_matrices_are_not_equal_m3() {
        //Matrix equality with different matrices3
        let m1 = [[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 8.0, 7.0]];
        let m2 = [[5.0, 6.0, 7.0], [9.0, 8.0, 7.0], [1.0, 2.0, 3.0]];
        assert_eq!(get_bool_equal_m3(m1, m2), false);
    }

    #[test]
    fn test_matrices_are_not_equal_m2() {
        //Matrix equality with different matrices2
        let m1 = [[1.0, 2.0], [5.0, 6.0]];
        let m2 = [[5.0, 6.0], [1.0, 2.0]];
        assert_eq!(get_bool_equal_m2(m1, m2), false);
    }

    #[test]
    fn test_matrix4_multiply() {
        //Multiplying two matrix4
        let m1 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ];
        let m2 = [
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ];
        let m3 = [
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ];
        assert_eq!(get_bool_equal_m4(matrix4_multiply(m1, m2), m3), true);
    }

    #[test]
    fn test_matrix4_tuple_multiply() {
        //Multiplying a matrix by a tuple
        let m1 = [
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let t = tuples::tuple(1.0, 2.0, 3.0, 1);
        let r = tuples::tuple(18.0, 24.0, 33.0, 1);
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&matrix4_tuple_multiply(m1, t), &r),
            true
        );
    }

    #[test]
    fn test_matrix4_multiply_identity() {
        //Multiplying two matrix4
        let m1 = [
            [0.0, 1.0, 2.0, 3.0],
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ];
        assert_eq!(
            get_bool_equal_m4(matrix4_multiply(m1, IDENTITY_MATRIX), m1),
            true
        );
    }

    #[test]
    fn test_matrix4_transpose() {
        //Transposing a Matrix
        let m1 = [
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ];
        let m2 = [
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ];
        assert_eq!(get_bool_equal_m4(matrix4_transpose(m1), m2), true);
    }

    #[test]
    fn test_matrix4_transpose_identity() {
        //Transposing the Identity Matrix
        assert_eq!(
            get_bool_equal_m4(matrix4_transpose(IDENTITY_MATRIX), IDENTITY_MATRIX),
            true
        );
    }

    #[test]
    fn test_matrix2_determinant() {
        //Calculating the determinant of a 2x2 matrix
        let m1 = [[1.0, 5.0], [-3.0, 2.0]];
        assert_eq!(
            tuples::get_bool_numbers_are_equal(matrix2_determinant(m1), 17.0),
            true
        );
    }

    /*
    #[test]
    fn test_matrix3_determinant() {
        //Calculating the determinant of a 3x3 matrix
        let m1 = [[1.0, 5.0, 0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]];
        let r = [[-3.0, 2.0], [0.0, 6.0]];
        assert_eq!(
            tuples::get_bool_equal_m4(matrix3_determinant(m1), r),
            true
        );
    }*/
}
