pub struct Tuple {
  x: f64,
  y: f64,
  z: f64,
  w: u32,
}



pub fn tuple(x:f64,y:f64,z:f64,w:u32) -> Tuple {
    Tuple {x:x,y:y,z:z,w:w}
}

pub fn point(x:f64,y:f64,z:f64) -> Tuple {
    Tuple {x:x, y:y, z:z, w:1}
}

pub fn vector(x:f64,y:f64,z:f64) -> Tuple {
    Tuple {x:x, y:y, z:z, w:0}
}

#[cfg(test)]
mod tests {
  use super::*;

    #[test]
    fn tuple_is_a_point() {
        //A tuple with w=1.0 is a point
        let a = tuple(4.3, -4.2, 3.1, 1);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1);
    }

    #[test]
    fn tuple_is_a_vector() {
        //A tuple with w=0.0 is a vector
        let a = tuple(4.3, -4.2, 3.1, 0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0);
    }

    #[test]
    fn point_is_a_tuple_w1() {
        //point() creates tuples with w=1
        let a = point(4.0, -4.0, 3.0);
        assert_eq!(a.x, 4.0);
        assert_eq!(a.y, -4.0);
        assert_eq!(a.z, 3.0);
        assert_eq!(a.w, 1);
    }

    #[test]
    fn vector_is_a_tuple_w0() {
        //test("vector() creates tuples with w=0"
        let a = vector(4.0, -4.0, 3.0);
        assert_eq!(a.x, 4.0);
        assert_eq!(a.y, -4.0);
        assert_eq!(a.z, 3.0);
        assert_eq!(a.w, 0);
    }
}
