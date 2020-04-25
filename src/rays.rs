use crate::matrices;
use crate::spheres;
use crate::transformations;
use crate::tuples;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: tuples::Point,
    pub direction: tuples::Vector,
}

impl Ray {
    pub fn discriminant(&self) -> spheres::Discriminant {
        let v_sphere_to_ray: tuples::Vector = self.origin.subtract(&tuples::POINT_ORIGIN);
        let a: f64 = self.direction.dot_product(&self.direction);
        let b: f64 = self.direction.dot_product(&v_sphere_to_ray) * 2.0;
        let c: f64 = v_sphere_to_ray.dot_product(&v_sphere_to_ray) - 1.0;
        let d: f64 = b * b - 4.0 * a * c;
        spheres::Discriminant {
            a: a,
            b: b,
            c: c,
            d: d,
        }
    }

    pub fn position(self, t: f64) -> tuples::Tuple {
        self.origin.add(&self.direction.multiply(&t))
    }

    pub fn transform(&self, m: matrices::Matrix4) -> Ray {
        let origin = transformations::transform_tuple_with_chain(&[m].to_vec(), &self.origin);
        let direction = transformations::transform_tuple_with_chain(&[m].to_vec(), &self.direction);
        ray(origin, direction)
    }
}

pub const RAY_NULL: Ray = Ray {
    origin: tuples::POINT_ORIGIN,
    direction: tuples::VECTOR_NULL,
};

pub fn ray(o: tuples::Point, d: tuples::Vector) -> Ray {
    Ray {
        origin: o,
        direction: d,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray() {
        //Creating and querying a ray
        let origin = tuples::point(1.0, 2.0, 3.0);
        let direction = tuples::vector(4.0, 5.0, 6.0);
        let r = ray(*&origin, *&direction);
        assert_eq!(r.origin.is_equal_to(&origin), true);
        assert_eq!(r.direction.is_equal_to(&direction), true);
    }

    #[test]
    fn test_point_from_distance() {
        //Computing a point from a distance
        let o = tuples::point(2.0, 3.0, 4.0);
        let d = tuples::vector(1.0, 0.0, 0.0);
        let r = ray(o, d);
        assert_eq!(
            r.position(0.0).is_equal_to(&tuples::point(2.0, 3.0, 4.0)),
            true
        );
        assert_eq!(
            r.position(1.0).is_equal_to(&tuples::point(3.0, 3.0, 4.0)),
            true
        );
        assert_eq!(
            r.position(-1.0).is_equal_to(&tuples::point(1.0, 3.0, 4.0)),
            true
        );
        assert_eq!(
            r.position(2.5).is_equal_to(&tuples::point(4.5, 3.0, 4.0)),
            true
        );
    }

    #[test]
    fn test_translate_ray() {
        //Translating a ray
        let o = tuples::point(1.0, 2.0, 3.0);
        let d = tuples::vector(0.0, 1.0, 0.0);
        let r = ray(o, d);
        let m = transformations::matrix4_translation(3.0, 4.0, 5.0);
        let r2 = r.transform(m);
        assert_eq!(r2.origin.is_equal_to(&tuples::point(4.0, 6.0, 8.0)), true);
        assert_eq!(
            r2.direction.is_equal_to(&tuples::vector(0.0, 1.0, 0.0)),
            true
        );
    }

    #[test]
    fn test_scaling_ray() {
        //Scaling a ray
        let o = tuples::point(1.0, 2.0, 3.0);
        let d = tuples::vector(0.0, 1.0, 0.0);
        let r = ray(o, d);
        let m = transformations::matrix4_scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(m);
        assert_eq!(r2.origin.is_equal_to(&tuples::point(2.0, 6.0, 12.0)), true);
        assert_eq!(
            r2.direction.is_equal_to(&tuples::vector(0.0, 3.0, 0.0)),
            true
        );
    }
}
