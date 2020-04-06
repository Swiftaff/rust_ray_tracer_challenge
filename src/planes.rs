use crate::intersections;
use crate::materials;
use crate::matrices;
use crate::rays;
use crate::shapes;
use crate::tuples;

pub fn plane() -> shapes::Shape {
    shapes::shape(shapes::ShapeType::Plane)
}

pub fn set_transform(mut s: shapes::Shape, t: matrices::Matrix4) -> shapes::Shape {
    s.transform = t;
    s
}

pub fn set_material(mut s: shapes::Shape, m: materials::Material) -> shapes::Shape {
    s.material = m;
    s
}

pub fn local_intersect(
    s: shapes::Shape,
    local_r: rays::Ray,
) -> Result<Vec<intersections::Intersection>, String> {
    if local_r.direction.y.abs() < tuples::EPSILON {
        return Err("No intersections".to_string());
    }
    let t = -1.0 * local_r.origin.y / local_r.direction.y;
    Ok(vec![intersections::intersection(t, s)])
}

pub fn local_normal_at() -> tuples::Vector {
    tuples::vector(0.0, 1.0, 0.0)
}

fn print_type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_normal_of_plane_is_constant_everywhere() {
        //The normal of a plane is constant everywhere
        let s = plane();
        let n1 = shapes::normal_at(s.clone(), tuples::point(0.0, 0.0, 0.0));
        let n2 = shapes::normal_at(s.clone(), tuples::point(10.0, 0.0, -10.0));
        let n3 = shapes::normal_at(s.clone(), tuples::point(-5.0, 0.0, 150.0));
        let result = tuples::vector(0.0, 1.0, 0.0);
        assert_eq!(tuples::get_bool_tuples_are_equal(&n1, &result), true);
        assert_eq!(tuples::get_bool_tuples_are_equal(&n2, &result), true);
        assert_eq!(tuples::get_bool_tuples_are_equal(&n3, &result), true);
    }

    #[test]
    fn test_intersect_ray_parallel_to_plane() {
        //Intersect with a ray parallel to the plane
        let s = plane();
        let r = rays::ray(tuples::point(0.0, 10.0, 0.0), tuples::vector(0.0, 0.0, 1.0));
        let x = shapes::intersect(s, r);
        match x {
            Err(e) => assert_eq!(e, "No intersections"),
            Ok(_xs) => {
                println!("Not possible in this test");
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn test_intersect_with_a_coplanar_ray() {
        //Intersect with a coplanar ray
        let s = plane();
        let r = rays::ray(tuples::point(0.0, 0.0, 0.0), tuples::vector(0.0, 0.0, 1.0));
        let x = shapes::intersect(s, r);
        match x {
            Err(e) => assert_eq!(e, "No intersections"),
            Ok(_xs) => {
                println!("Not possible in this test");
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn test_ray_intersects_plane_from_above() {
        //A ray intersects a plane from above
        let r = rays::ray(tuples::point(0.0, 1.0, 0.0), tuples::vector(0.0, -1.0, 0.0));
        let s = plane();
        let x = shapes::intersect(s.clone(), r);
        match x {
            Err(_) => {
                println!("Not possible in this test");
                assert_eq!(false, true);
            }
            Ok(xs) => {
                assert_eq!(xs.len(), 1);
                assert_eq!(xs[0].object.id, s.id);
            }
        }
    }

    #[test]
    fn test_ray_intersects_plane_from_below() {
        //A ray intersects a plane from below
        let r = rays::ray(tuples::point(0.0, -1.0, 0.0), tuples::vector(0.0, 1.0, 0.0));
        let s = plane();
        let x = shapes::intersect(s.clone(), r);
        match x {
            Err(_) => {
                println!("Not possible in this test");
                assert_eq!(false, true);
            }
            Ok(xs) => {
                assert_eq!(xs.len(), 1);
                assert_eq!(xs[0].object.id, s.id);
            }
        }
    }

    #[test]
    fn test_a_plane_is_a_shape() {
        //A plane is a shape
        let s = plane();
        let type_name = print_type_of(&s);
        assert_eq!(type_name, "rust_ray_tracer_challenge::shapes::Shape");
    }
}
