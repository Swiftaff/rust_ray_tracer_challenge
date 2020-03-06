pub struct Tuple {
  x: f64,
  y: f64,
  z: f64,
  w: u32,
}

pub fn tuple(x:f64,y:f64,z:f64,w:u32) -> Tuple {
    Tuple {x:x,y:y,z:z,w:w}
}

#[cfg(test)]
mod tests {
  use super::*;

    #[test]
        fn test_tuple() {
        let a = tuple(4.3, -4.2, 3.1, 1);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1);
    }
}
