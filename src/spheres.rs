use std::f64::consts::PI;
use uuid::Uuid;

use crate::intersections;
use crate::materials;
use crate::matrices;
use crate::rays;
use crate::shapes;
use crate::transformations;
use crate::tuples;

#[derive(Debug, Clone)]
pub struct Discriminant {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}

pub fn sphere() -> shapes::Shape {
    shapes::shape(shapes::ShapeType::ShapeSphere)
}

pub fn set_transform(mut s: shapes::Shape, t: matrices::Matrix4) -> shapes::Shape {
    s.transform = t;
    s
}

pub fn set_material(mut s: shapes::Shape, m: materials::Material) -> shapes::Shape {
    s.material = m;
    s
}

pub fn discriminant(s: shapes::Shape, ray: rays::Ray) -> Discriminant {
    let v_sphere_to_ray: tuples::Vector =
        tuples::tuple_subtract(&ray.origin, &tuples::POINT_ORIGIN);
    let a: f64 = tuples::vector_dot_product(&ray.direction, &ray.direction);
    let b: f64 = tuples::vector_dot_product(&ray.direction, &v_sphere_to_ray) * 2.0;
    let c: f64 = tuples::vector_dot_product(&v_sphere_to_ray, &v_sphere_to_ray) - 1.0;
    let d: f64 = b * b - 4.0 * a * c;
    Discriminant {
        a: a,
        b: b,
        c: c,
        d: d,
    }
}

pub fn intersect(
    s: shapes::Shape,
    r: rays::Ray,
) -> Result<Vec<intersections::Intersection>, String> {
    let r2: rays::Ray = rays::ray_transform(r, matrices::matrix4_inverse(s.transform));
    let disc: Discriminant = discriminant(s.clone(), r2);
    if disc.d < 0.0 {
        Err("No intersections".to_string())
    } else {
        //hits
        let t1 = (-disc.b - disc.d.sqrt()) / (2.0 * disc.a);
        let t2 = (-disc.b + disc.d.sqrt()) / (2.0 * disc.a);
        let i1 = if t1 < t2 {
            intersections::intersection(t1, s.clone())
        } else {
            intersections::intersection(t2, s.clone())
        };
        let i2 = if t1 < t2 {
            intersections::intersection(t2, s.clone())
        } else {
            intersections::intersection(t1, s.clone())
        };
        let xs: Vec<intersections::Intersection> = intersections::intersection_list(vec![i1, i2]);
        Ok(xs)
    }
}

pub fn local_intersect(
    s: shapes::Shape,
    local_r: rays::Ray,
) -> Result<Vec<intersections::Intersection>, String> {
    let disc: Discriminant = discriminant(s.clone(), local_r);
    if disc.d < 0.0 {
        Err("No intersections".to_string())
    } else {
        //hits
        let t1 = (-disc.b - disc.d.sqrt()) / (2.0 * disc.a);
        let t2 = (-disc.b + disc.d.sqrt()) / (2.0 * disc.a);
        let i1 = if t1 < t2 {
            intersections::intersection(t1, s.clone())
        } else {
            intersections::intersection(t2, s.clone())
        };
        let i2 = if t1 < t2 {
            intersections::intersection(t2, s.clone())
        } else {
            intersections::intersection(t1, s.clone())
        };
        let xs: Vec<intersections::Intersection> = intersections::intersection_list(vec![i1, i2]);
        Ok(xs)
    }
}

pub fn normal_at(s: shapes::Shape, world_point: tuples::Point) -> tuples::Vector {
    let object_point: tuples::Point =
        matrices::matrix4_tuple_multiply(matrices::matrix4_inverse(s.transform), world_point);
    let object_normal: tuples::Vector =
        tuples::tuple_subtract(&object_point, &tuples::POINT_ORIGIN);
    let mut world_normal: tuples::Vector = matrices::matrix4_tuple_multiply(
        matrices::matrix4_transpose(matrices::matrix4_inverse(s.transform)),
        object_normal,
    );
    world_normal.w = 0;
    tuples::vector_normalize(&world_normal)
}

pub fn local_normal_at(local_point: tuples::Point) -> tuples::Vector {
    tuples::tuple_subtract(&local_point, &tuples::POINT_ORIGIN)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spheres_have_unique_ids() {
        //Spheres have unique IDs
        let s1 = sphere();
        let s2 = sphere();
        let s3 = sphere();
        let s4 = sphere();
        assert_eq!(s1.id == s2.id, false);
        assert_eq!(s2.id == s3.id, false);
        assert_eq!(s3.id == s4.id, false);
        assert_eq!(s4.id == s1.id, false);
    }

    #[test]
    fn test_ray_intersects_sphere_at_two_points() {
        //A ray intersects a sphere at two points
        let r = rays::ray(tuples::point(0.0, 0.0, -5.0), tuples::vector(0.0, 0.0, 1.0));
        let s = sphere();
        let x = intersect(s, r);
        match x {
            Err(e) => println!("XS Error: {}", e),
            Ok(xs) => {
                assert_eq!(xs.len() == 2, true);
                assert_eq!(xs[0].t == 4.0, true);
                assert_eq!(xs[1].t == 6.0, true);
            }
        }
    }

    #[test]
    fn test_ray_intersects_sphere_at_two_identical_points_tangent() {
        //A ray intersects a sphere at a tangent
        let r = rays::ray(tuples::point(0.0, 1.0, -5.0), tuples::vector(0.0, 0.0, 1.0));
        let s = sphere();
        let x = intersect(s, r);
        match x {
            Err(e) => println!("XS Error: {}", e),
            Ok(xs) => {
                assert_eq!(xs.len() == 2, true);
                assert_eq!(xs[0].t == 5.0, true);
                assert_eq!(xs[1].t == 5.0, true);
            }
        }
    }

    #[test]
    fn test_ray_misses_sphere() {
        //A ray misses a sphere
        let r = rays::ray(tuples::point(0.0, 2.0, -5.0), tuples::vector(0.0, 0.0, 1.0));
        let s = sphere();
        let x = intersect(s, r);
        match x {
            Err(e) => assert_eq!(e == "No intersections", true),
            Ok(xs) => {
                println!("Error");
            }
        }
    }

    #[test]
    fn test_ray_originates_inside_sphere() {
        //A ray originates inside a sphere
        let r = rays::ray(tuples::point(0.0, 0.0, 0.0), tuples::vector(0.0, 0.0, 1.0));
        let s = sphere();
        let x = intersect(s, r);
        match x {
            Err(e) => println!("XS Error: {}", e),
            Ok(xs) => {
                assert_eq!(xs.len() == 2, true);
                assert_eq!(xs[0].t == -1.0, true);
                assert_eq!(xs[1].t == 1.0, true);
            }
        }
    }

    #[test]
    fn test_sphere_is_behind_ray() {
        //A sphere is behind a ray
        let r = rays::ray(tuples::point(0.0, 0.0, 5.0), tuples::vector(0.0, 0.0, 1.0));
        let s = sphere();
        let x = intersect(s, r);
        match x {
            Err(e) => println!("XS Error: {}", e),
            Ok(xs) => {
                assert_eq!(xs.len() == 2, true);
                assert_eq!(xs[0].t == -6.0, true);
                assert_eq!(xs[1].t == -4.0, true);
            }
        }
    }

    #[test]
    fn test_intersect_sets_object_on_intersection() {
        //Intersect sets the object on the intersection
        let r = rays::ray(tuples::point(0.0, 0.0, -5.0), tuples::vector(0.0, 0.0, 1.0));
        let s = sphere();
        let s1 = s.clone();
        let s2 = s.clone();
        let x = intersect(s, r);
        match x {
            Err(e) => println!("XS Error: {}", e),
            Ok(xs) => {
                assert_eq!(xs.len() == 2, true);
                assert_eq!(xs[0].object.id == s1.clone().id, true);
                assert_eq!(xs[1].object.id == s2.clone().id, true);
            }
        }
    }

    #[test]
    fn test_spheres_default_transformation() {
        //A sphere's default transformation
        let s = sphere();
        assert_eq!(
            matrices::get_bool_equal_m4(s.transform, matrices::IDENTITY_MATRIX),
            true
        );
    }

    #[test]
    fn test_change_spheres_transformation() {
        //Changing a sphere's transformation
        let s = sphere();
        let t = transformations::matrix4_translation(2.0, 3.0, 4.0);
        let s2 = set_transform(s.clone(), t.clone());
        assert_eq!(matrices::get_bool_equal_m4(s2.transform, t), true);
    }

    #[test]
    fn test_intersect_scaled_sphere_with_ray() {
        //Intersecting a scaled sphere with a ray
        let r = rays::ray(tuples::point(0.0, 0.0, -5.0), tuples::vector(0.0, 0.0, 1.0));
        let s = sphere();
        let t = transformations::matrix4_scaling(2.0, 2.0, 2.0);
        let s2 = set_transform(s, t);
        let x = intersect(s2, r);
        match x {
            Err(e) => println!("XS Error: {}", e),
            Ok(xs) => {
                assert_eq!(xs.len() == 2, true);
                assert_eq!(xs[0].t == 3.0, true);
                assert_eq!(xs[1].t == 7.0, true);
            }
        }
    }

    #[test]
    fn test_intersect_translated_sphere_with_ray() {
        //Intersecting a translated sphere with a ray
        let r = rays::ray(tuples::point(0.0, 0.0, -5.0), tuples::vector(0.0, 0.0, 1.0));
        let s = sphere();
        let t = transformations::matrix4_translation(5.0, 0.0, 0.0);
        let s2 = set_transform(s, t);
        let x = intersect(s2, r);
        match x {
            Err(e) => assert_eq!(e == "No intersections", true),
            Ok(xs) => {
                println!("Error");
            }
        }
    }

    //normal_at
    #[test]
    fn test_sphere_normal_at_point_x_axis() {
        //The normal on a sphere at a point on the x axis
        let s = sphere();
        let n = normal_at(s, tuples::point(1.0, 0.0, 0.0));
        let r = tuples::vector(1.0, 0.0, 0.0);
        assert_eq!(tuples::get_bool_tuples_are_equal(&n, &r), true);
    }

    #[test]
    fn test_sphere_normal_at_point_y_axis() {
        //The normal on a sphere at a point on the y axis
        let s = sphere();
        let n = normal_at(s, tuples::point(0.0, 1.0, 0.0));
        let r = tuples::vector(0.0, 1.0, 0.0);
        assert_eq!(tuples::get_bool_tuples_are_equal(&n, &r), true);
    }

    #[test]
    fn test_sphere_normal_at_point_z_axis() {
        //The normal on a sphere at a point on the z axis
        let s = sphere();
        let n = normal_at(s, tuples::point(0.0, 0.0, 1.0));
        let r = tuples::vector(0.0, 0.0, 1.0);
        assert_eq!(tuples::get_bool_tuples_are_equal(&n, &r), true);
    }

    #[test]
    fn test_sphere_normal_at_nonaxial_point() {
        //The normal on a sphere at a nonaxial point
        let s = sphere();
        let n = normal_at(
            s,
            tuples::point(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
            ),
        );
        let r = tuples::vector(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        );
        assert_eq!(tuples::get_bool_tuples_are_equal(&n, &r), true);
    }

    #[test]
    fn test_sphere_normal_is_normalized_vector() {
        //The normal is a normalized vector
        let s = sphere();
        let n = normal_at(
            s,
            tuples::point(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
            ),
        );
        let r = tuples::vector(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        );
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&r, &tuples::vector_normalize(&r)),
            true
        );
    }

    #[test]
    fn test_normal_on_translated_sphere() {
        //Computing the normal on a translated sphere
        let s = sphere();
        let s2 = set_transform(s, transformations::matrix4_translation(0.0, 1.0, 0.0));
        let n = normal_at(s2, tuples::point(0.0, 1.70711, -0.70711));
        let r = tuples::vector(0.0, 0.70711, -0.70711);
        assert_eq!(tuples::get_bool_tuples_are_equal(&r, &n), true);
    }

    #[test]
    fn test_normal_on_transformed_sphere() {
        //Computing the normal on a transformed sphere
        let s = sphere();
        let m1 = transformations::matrix4_scaling(1.0, 0.5, 1.0);
        let m2 = transformations::matrix4_rotation_z_rad(PI / 5.0);
        let m = matrices::matrix4_multiply(m1, m2);
        let s2 = set_transform(s, m);
        let n = normal_at(
            s2,
            tuples::point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0),
        );
        let r = tuples::vector(0.0, 0.97014, -0.24254);
        assert_eq!(tuples::get_bool_tuples_are_equal(&r, &n), true);
    }

    #[test]
    fn test_assign_material_to_sphere() {
        //A sphere may be assigned a material
        let s = sphere();
        assert_eq!(s.material.color.red == tuples::COLOR_WHITE.red, true);
        assert_eq!(s.material.ambient == 0.1, true);
        assert_eq!(s.material.diffuse == 0.9, true);
        assert_eq!(s.material.specular == 0.9, true);
        assert_eq!(s.material.shininess == 200.0, true);
        let mut m = materials::MATERIAL_DEFAULT;
        m.ambient = 1.0;
        let s2 = set_material(s, m);
        assert_eq!(s2.material.ambient == 1.0, true);
    }
}
