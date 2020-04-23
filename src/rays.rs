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
        let v_sphere_to_ray: tuples::Vector =
            tuples::tuple_subtract(&self.origin, &tuples::POINT_ORIGIN);
        let a: f64 = tuples::vector_dot_product(&self.direction, &self.direction);
        let b: f64 = tuples::vector_dot_product(&self.direction, &v_sphere_to_ray) * 2.0;
        let c: f64 = tuples::vector_dot_product(&v_sphere_to_ray, &v_sphere_to_ray) - 1.0;
        let d: f64 = b * b - 4.0 * a * c;
        spheres::Discriminant {
            a: a,
            b: b,
            c: c,
            d: d,
        }
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

pub fn position(ray: Ray, t: f64) -> tuples::Tuple {
    tuples::tuple_add(&ray.origin, &tuples::tuple_multiply(&ray.direction, &t))
}

pub fn ray_transform(r: &Ray, m: matrices::Matrix4) -> Ray {
    let o = r.origin;
    let d = r.direction;
    let origin = transformations::transform_tuple_with_chain(&[m].to_vec(), &o);
    let direction = transformations::transform_tuple_with_chain(&[m].to_vec(), &d);
    ray(origin, direction)
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
        assert_eq!(tuples::get_bool_tuples_are_equal(&r.origin, &origin), true);
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&r.direction, &direction),
            true
        );
    }

    #[test]
    fn test_point_from_distance() {
        //Computing a point from a distance
        let o = tuples::point(2.0, 3.0, 4.0);
        let d = tuples::vector(1.0, 0.0, 0.0);
        let r = ray(o, d);
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&position(*&r, 0.0), &tuples::point(2.0, 3.0, 4.0)),
            true
        );
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&position(*&r, 1.0), &tuples::point(3.0, 3.0, 4.0)),
            true
        );
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&position(*&r, -1.0), &tuples::point(1.0, 3.0, 4.0)),
            true
        );
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&position(*&r, 2.5), &tuples::point(4.5, 3.0, 4.0)),
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
        let r2 = ray_transform(&r, m);
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&r2.origin, &tuples::point(4.0, 6.0, 8.0)),
            true
        );
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&r2.direction, &tuples::vector(0.0, 1.0, 0.0)),
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
        let r2 = ray_transform(&r, m);
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&r2.origin, &tuples::point(2.0, 6.0, 12.0)),
            true
        );
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&r2.direction, &tuples::vector(0.0, 3.0, 0.0)),
            true
        );
    }
}
