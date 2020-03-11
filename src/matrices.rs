const IDENTITY_MATRIX: Matrix4 = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

type Matrix2 = [[f64; 2]; 2];
type Matrix3 = [[f64; 3]; 3];
type Matrix4 = [[f64; 4]; 4];

pub fn getM2(m: Matrix2, y: usize, x: usize) -> f64 {
    m[y][x]
}

pub fn getM3(m: Matrix3, y: usize, x: usize) -> f64 {
    m[y][x]
}
pub fn getM4(m: Matrix4, y: usize, x: usize) -> f64 {
    m[y][x]
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
}
