#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub id: String,
    pub transform: matrices::Matrix4,
    pub material: u32,
}
pub fn tuple(x: f64, y: f64, z: f64, w: u32) -> Tuple {
    Tuple {
        x: x,
        y: y,
        z: z,
        w: w,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_is_a_tuplew1() {
        //A tuple with w=1.0 is a point
        let a = tuple(4.3, -4.2, 3.1, 1);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1);
    }

}
