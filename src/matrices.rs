use crate::tuples;

pub const IDENTITY_MATRIX: Matrix4 = Matrix4([
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
]);

pub struct Matrix2([[f64; 2]; 2]);
pub struct Matrix3([[f64; 3]; 3]);

#[derive(Debug, Copy, Clone)]
pub struct Matrix4(pub [[f64; 4]; 4]);

pub fn create_matrix4() -> Matrix4 {
    let row: [f64; 4] = [0.0; 4];
    let arr: [[f64; 4]; 4] = [row; 4];
    Matrix4(arr)
}

pub fn create_matrix3() -> Matrix3 {
    let row: [f64; 3] = [0.0; 3];
    let arr: [[f64; 3]; 3] = [row; 3];
    Matrix3(arr)
}

pub fn create_matrix2() -> Matrix2 {
    let row: [f64; 2] = [0.0; 2];
    let arr: [[f64; 2]; 2] = [row; 2];
    Matrix2(arr)
}

impl Matrix2 {
    pub fn get_at(&self, y: usize, x: usize) -> f64 {
        self.0[y][x]
    }

    pub fn is_equal_to(&self, m2: &Matrix2) -> bool {
        let mut are_equal = true;
        let rows = self.0.len();
        let cols = self.0[0].len();
        for y in 0..rows {
            for x in 0..cols {
                if tuples::get_bool_numbers_are_equal(&self.0[y][x], &m2.0[y][x]) == false {
                    are_equal = false;
                }
            }
        }
        are_equal
    }

    pub fn determinant(&self) -> f64 {
        self.0[0][0] * self.0[1][1] - self.0[0][1] * self.0[1][0]
    }
}

impl Matrix3 {
    pub fn get_at(&self, y: usize, x: usize) -> f64 {
        self.0[y][x]
    }

    pub fn is_equal_to(&self, m2: &Matrix3) -> bool {
        let mut are_equal = true;
        let rows = self.0.len();
        let cols = self.0[0].len();
        for y in 0..rows {
            for x in 0..cols {
                if tuples::get_bool_numbers_are_equal(&self.0[y][x], &m2.0[y][x]) == false {
                    are_equal = false;
                }
            }
        }
        are_equal
    }

    pub fn determinant(&self) -> f64 {
        let mut det = 0.0;
        for col in 0..3 {
            det = det + self.0[0][col] * self.cofactor(&0, &col);
        }
        det
    }

    pub fn minor(&self, row_to_delete: &usize, col_to_delete: &usize) -> f64 {
        self.submatrix2(row_to_delete, col_to_delete).determinant()
    }

    pub fn cofactor(&self, row: &usize, col: &usize) -> f64 {
        let m1 = self.minor(row, col);
        if (row + col) % 2 == 1 {
            -1.0 * m1
        } else {
            m1
        }
    }

    pub fn submatrix2(&self, row_to_delete: &usize, col_to_delete: &usize) -> Matrix2 {
        let mut result = create_matrix2();
        for y in 0..3 {
            for x in 0..3 {
                if &y != row_to_delete {
                    if &x != col_to_delete {
                        let xx = if &x > col_to_delete { x - 1 } else { x };
                        let yy = if &y > row_to_delete { y - 1 } else { y };
                        result.0[yy][xx] = self.0[y][x];
                    }
                }
            }
        }
        result
    }
}

impl Matrix4 {
    pub fn get_at(&self, y: usize, x: usize) -> f64 {
        self.0[y][x]
    }

    pub fn is_equal_to(&self, m2: &Matrix4) -> bool {
        let mut are_equal = true;
        let rows = self.0.len();
        let cols = self.0[0].len();
        for y in 0..rows {
            for x in 0..cols {
                if tuples::get_bool_numbers_are_equal(&self.0[y][x], &m2.0[y][x]) == false {
                    are_equal = false;
                }
            }
        }
        are_equal
    }

    pub fn is_invertible(&self) -> bool {
        !tuples::get_bool_numbers_are_equal(&self.determinant(), &0.0)
    }

    pub fn multiply(&self, m2: &Matrix4) -> Matrix4 {
        let mut result = create_matrix4();
        for y in 0..4 {
            for x in 0..4 {
                let mut this_result = 0.0;
                for xx in 0..4 {
                    this_result = this_result + self.0[y][xx] * m2.0[xx][x];
                }
                result.0[y][x] = this_result;
            }
        }
        result
    }

    pub fn transpose(&self) -> Matrix4 {
        let mut result = create_matrix4();
        for y in 0..4 {
            for x in 0..4 {
                result.0[y][x] = self.0[x][y];
            }
        }
        result
    }

    pub fn determinant(&self) -> f64 {
        let mut det = 0.0;
        for col in 0..4 {
            det = det + self.0[0][col] * self.cofactor(&0, &col);
        }
        det
    }

    pub fn minor(&self, row_to_delete: &usize, col_to_delete: &usize) -> f64 {
        self.submatrix3(row_to_delete, col_to_delete).determinant()
    }

    pub fn cofactor(&self, row: &usize, col: &usize) -> f64 {
        let m1 = self.minor(row, col);
        if (row + col) % 2 == 1 {
            -1.0 * m1
        } else {
            m1
        }
    }

    pub fn submatrix3(&self, row_to_delete: &usize, col_to_delete: &usize) -> Matrix3 {
        let mut result = create_matrix3();
        for y in 0..4 {
            for x in 0..4 {
                if &y != row_to_delete {
                    if &x != col_to_delete {
                        let xx = if &x > col_to_delete { x - 1 } else { x };
                        let yy = if &y > row_to_delete { y - 1 } else { y };
                        result.0[yy][xx] = self.0[y][x];
                    }
                }
            }
        }
        result
    }

    pub fn tuple_multiply(&self, t: &tuples::Tuple) -> tuples::Tuple {
        let mut result = [[0.0], [0.0], [0.0], [0.0]];
        let m2 = [[t.x], [t.y], [t.z], [t.w as f64]];
        for y in 0..4 {
            for x in 0..1 {
                let mut this_result = 0.0;
                for xx in 0..4 {
                    this_result = this_result + self.0[y][xx] * m2[xx][x];
                }
                result[y][x] = this_result;
            }
        }
        tuples::Tuple {
            x: result[0][0],
            y: result[1][0],
            z: result[2][0],
            w: result[3][0] as u32,
        }
    }

    pub fn inverse(&self) -> Matrix4 {
        if self.is_invertible() {
            let mut result = create_matrix4();
            for y in 0..4 {
                for x in 0..4 {
                    let c = self.cofactor(&y, &x);
                    result.0[x][y] = c / self.determinant();
                }
            }
            result
        } else {
            *self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix4() {
        //Creating a matrix
        let m = Matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);
        assert_eq!(m.get_at(0, 0), 1.0);
        assert_eq!(m.get_at(0, 3), 4.0);
        assert_eq!(m.get_at(1, 0), 5.5);
        assert_eq!(m.get_at(1, 2), 7.5);
        assert_eq!(m.get_at(2, 2), 11.0);
        assert_eq!(m.get_at(3, 0), 13.5);
        assert_eq!(m.get_at(3, 2), 15.5);
    }

    #[test]
    fn test_matrix3() {
        //A 3x3 matrix ought to be representable
        let m = Matrix3([[1.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]]);
        assert_eq!(m.get_at(0, 0), 1.0);
        assert_eq!(m.get_at(0, 2), 3.0);
        assert_eq!(m.get_at(1, 0), 5.5);
        assert_eq!(m.get_at(1, 2), 7.5);
        assert_eq!(m.get_at(2, 2), 11.0);
        assert_eq!(m.get_at(2, 0), 9.0);
    }

    #[test]
    fn test_matrix2() {
        //A 2x2 matrix ought to be representable
        let m = Matrix2([[-3.0, 5.0], [1.0, -2.0]]);
        assert_eq!(m.get_at(0, 0), -3.0);
        assert_eq!(m.get_at(0, 1), 5.0);
        assert_eq!(m.get_at(1, 0), 1.0);
        assert_eq!(m.get_at(1, 1), -2.0);
    }

    #[test]
    fn test_matrices_are_equal_m4() {
        //Matrix equality with identical matrices4
        let m1 = Matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        assert_eq!(m1.is_equal_to(&m2), true);
    }

    #[test]
    fn test_matrices_are_equal_m3() {
        //Matrix equality with identical matrices3
        let m1 = Matrix3([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 8.0, 7.0]]);
        let m2 = Matrix3([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 8.0, 7.0]]);
        assert_eq!(m1.is_equal_to(&m2), true);
    }

    #[test]
    fn test_matrices_are_equal_m2() {
        //Matrix equality with identical matrices2
        let m1 = Matrix2([[1.0, 2.0], [5.0, 6.0]]);
        let m2 = Matrix2([[1.0, 2.0], [5.0, 6.0]]);
        assert_eq!(m1.is_equal_to(&m2), true);
    }

    #[test]
    fn test_matrices_are_not_equal_m4() {
        //Matrix equality with different matrices4
        let m1 = Matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Matrix4([
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
            [1.0, 2.0, 3.0, 4.0],
        ]);
        assert_eq!(m1.is_equal_to(&m2), false);
    }

    #[test]
    fn test_matrices_are_not_equal_m3() {
        //Matrix equality with different matrices3
        let m1 = Matrix3([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 8.0, 7.0]]);
        let m2 = Matrix3([[5.0, 6.0, 7.0], [9.0, 8.0, 7.0], [1.0, 2.0, 3.0]]);
        assert_eq!(m1.is_equal_to(&m2), false);
    }

    #[test]
    fn test_matrices_are_not_equal_m2() {
        //Matrix equality with different matrices2
        let m1 = Matrix2([[1.0, 2.0], [5.0, 6.0]]);
        let m2 = Matrix2([[5.0, 6.0], [1.0, 2.0]]);
        assert_eq!(m1.is_equal_to(&m2), false);
    }

    #[test]
    fn test_matrix4_multiply() {
        //Multiplying two matrix4
        let m1 = Matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Matrix4([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let m3 = Matrix4([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);
        assert_eq!(m1.multiply(&m2).is_equal_to(&m3), true);
    }

    #[test]
    fn test_matrix4_tuple_multiply() {
        //Multiplying a matrix by a tuple
        let m1 = Matrix4([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let t = tuples::tuple(1.0, 2.0, 3.0, 1);
        let r = tuples::tuple(18.0, 24.0, 33.0, 1);
        assert_eq!(m1.tuple_multiply(&t).is_equal_to(&r), true);
    }

    #[test]
    fn test_matrix4_multiply_identity() {
        //Multiplying two matrix4
        let m1 = Matrix4([
            [0.0, 1.0, 2.0, 3.0],
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);
        assert_eq!(m1.multiply(&IDENTITY_MATRIX).is_equal_to(&m1), true);
    }

    #[test]
    fn test_matrix4_transpose() {
        //Transposing a Matrix
        let m1 = Matrix4([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);
        let m2 = Matrix4([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);
        assert_eq!(m1.transpose().is_equal_to(&m2), true);
    }

    #[test]
    fn test_matrix4_transpose_identity() {
        //Transposing the Identity Matrix
        assert_eq!(
            IDENTITY_MATRIX.transpose().is_equal_to(&IDENTITY_MATRIX),
            true
        );
    }

    #[test]
    fn test_matrix2_determinant() {
        //Calculating the determinant of a 2x2 matrix
        let m1 = Matrix2([[1.0, 5.0], [-3.0, 2.0]]);
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&m1.determinant(), &17.0),
            true
        );
    }

    #[test]
    fn test_matrix3_submatrix2() {
        //A submatrix of 3x3 matrix is a 2x2 matrix
        let m1 = Matrix3([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
        let r = Matrix2([[-3.0, 2.0], [0.0, 6.0]]);
        assert_eq!(m1.submatrix2(&0, &2).is_equal_to(&r), true);
    }

    #[test]
    fn test_matrix4_submatrix3() {
        //A submatrix of 4x4 matrix is a 3x3 matrix
        let m1 = Matrix4([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        let r = Matrix3([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);
        assert_eq!(m1.submatrix3(&2, &1).is_equal_to(&r), true);
    }

    #[test]
    fn test_matrix3_minor() {
        //Calculating a minor of a 3 x 3 matrix
        let m = Matrix3([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        let s = m.submatrix2(&1, &0);
        let d = s.determinant();
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&m.minor(&1, &0), &d),
            true
        );
        assert_eq!(tuples::get_bool_numbers_are_equal(&d, &25.0), true);
    }

    #[test]
    fn test_matrix3_cofactor() {
        //Calculating a cofactor of a 3 x 3 matrix
        let m = Matrix3([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&m.minor(&0, &0), &-12.0),
            true
        );
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&m.cofactor(&0, &0), &-12.0),
            true
        );
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&m.minor(&1, &0), &25.0),
            true
        );
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&m.cofactor(&1, &0), &-25.0),
            true
        );
    }

    #[test]
    fn test_matrix3_determinant() {
        //Calculating the determinant of a 3 x 3 matrix
        let m1 = Matrix3([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&m1.cofactor(&0, &0), &56.0),
            true
        );
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&m1.cofactor(&0, &1), &12.0),
            true
        );
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&m1.cofactor(&0, &2), &-46.0),
            true
        );
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&m1.determinant(), &-196.0),
            true
        );
    }

    #[test]
    fn test_matrix4_determinant() {
        //Calculating the determinant of a 4 x 4 matrix
        let m1 = Matrix4([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&m1.cofactor(&0, &0), &690.0),
            true
        );
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&m1.cofactor(&0, &1), &447.0),
            true
        );
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&m1.cofactor(&0, &2), &210.0),
            true
        );
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&m1.cofactor(&0, &3), &51.0),
            true
        );
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&m1.determinant(), &-4071.0),
            true
        );
    }

    #[test]
    fn test_matrix4_is_invertible() {
        //Testing an invertible matrix for invertability
        let m1 = Matrix4([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&m1.determinant(), &-2120.0),
            true
        );
        assert_eq!(m1.is_invertible(), true)
    }

    #[test]
    fn test_matrix4_is_invertible_not() {
        //Testing a noninvertible matrix for invertability
        let m1 = Matrix4([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&m1.determinant(), &0.0),
            true
        );
        assert_eq!(m1.is_invertible(), false)
    }

    #[test]
    fn test_matrix4_inverse() {
        //Calculating the inverse of a matrix
        let m1 = Matrix4([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);
        let m2 = m1.inverse();
        let result = Matrix4([
            [0.21805, 0.45113, 0.2406, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&m1.determinant(), &532.0),
            true
        );
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&m1.cofactor(&2, &3), &-160.0),
            true
        );
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&m2.0[3][2], &(-160.0 / 532.0)),
            true
        );
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&m1.cofactor(&3, &2), &105.0),
            true
        );
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&m2.0[2][3], &(105.0 / 532.0)),
            true
        );
        assert_eq!(m2.is_equal_to(&result), true);
    }

    #[test]
    fn test_matrix4_inverse_again() {
        //Calculating the inverse of another matrix
        let m1 = Matrix4([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);
        let m2 = m1.inverse();
        let result = Matrix4([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.4359, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);
        assert_eq!(m2.is_equal_to(&result), true);
    }

    #[test]
    fn test_matrix4_inverse_third() {
        //Calculating the inverse of another matrix
        let m1 = Matrix4([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);
        let m2 = m1.inverse();
        let result = Matrix4([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.1463, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
        ]);
        assert_eq!(m2.is_equal_to(&result), true);
    }

    #[test]
    fn test_matrix4_multiply_by_inverse() {
        //Multiplying a product by its inverse
        let m1 = Matrix4([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);
        let m2 = Matrix4([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);
        let m3 = m1.multiply(&m2);
        assert_eq!(m3.multiply(&m2.inverse()).is_equal_to(&m1), true);
    }
}
