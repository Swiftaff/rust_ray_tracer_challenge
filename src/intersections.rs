use std::cmp::Ordering;

use crate::matrices;
use crate::rays;
use crate::spheres;
use crate::tuples;

pub fn comp_default() -> Comps {
    Comps {
        t: 0.0,
        object: spheres::sphere(),
        point: tuples::point(0.0, 0.0, 0.0),
        eyev: tuples::vector(0.0, 0.0, 0.0),
        normalv: tuples::vector(0.0, 0.0, 0.0),
        inside: false,
    }
}

#[derive(Debug, Clone)]
pub struct Comps {
    pub t: f64,
    pub object: spheres::Sphere,
    pub point: tuples::Point,
    pub eyev: tuples::Vector,
    pub normalv: tuples::Vector,
    pub inside: bool,
}

#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: spheres::Sphere,
}

pub fn intersection(t: f64, object: spheres::Sphere) -> Intersection {
    Intersection {
        t: t,
        object: object,
    }
}

pub fn intersection_list(mut xs: Vec<Intersection>) -> Vec<Intersection> {
    xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Ordering::Equal));
    xs
}

pub fn hit(xs: Vec<Intersection>) -> Result<Intersection, &'static str> {
    let mut theHit: i32 = -1;
    for index in 0..xs.len() {
        if theHit == -1 && xs[index].t >= 0.0 {
            theHit = index as i32;
        }
    }
    if theHit == -1 {
        Err("No hit")
    } else {
        Ok(xs[theHit as usize].clone())
    }
}

pub fn prepare_computations(i: Intersection, r: rays::Ray) -> Comps {
    let mut comps: Comps = comp_default();
    comps.t = i.t;
    comps.object = i.object;
    comps.point = rays::position(r, comps.t);
    comps.eyev = tuples::tuple_multiply(r.direction, -1.0);
    comps.normalv = spheres::normal_at(comps.clone().object, comps.clone().point);
    if tuples::vector_dot_product(&comps.normalv, &comps.eyev) < 0.0 {
        comps.inside = true;
        comps.normalv = tuples::tuple_multiply(comps.normalv, -1.0);
    }
    comps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_has_t_and_object() {
        //An intersection encapsulates t and object
        let s = spheres::sphere();
        let i = intersection(3.5, s);
        assert_eq!(tuples::get_bool_numbers_are_equal(i.t, 3.5), true);
        assert_eq!(
            matrices::get_bool_equal_m4(i.object.transform, matrices::IDENTITY_MATRIX),
            true
        );
        assert_eq!(
            tuples::get_bool_colors_are_equal(&i.object.material.color, &tuples::COLOR_WHITE),
            true
        );
    }

    #[test]
    fn test_aggregating_intersections() {
        //Aggregating intersections
        let s = spheres::sphere();
        let i1 = intersection(1.0, s.clone());
        let i2 = intersection(2.0, s.clone());
        let xs = intersection_list(vec![i2, i1]);
        assert_eq!(xs.len() == 2, true);
        assert_eq!(tuples::get_bool_numbers_are_equal(xs[0].t, 1.0), true);
        assert_eq!(tuples::get_bool_numbers_are_equal(xs[1].t, 2.0), true);
    }

    #[test]
    fn test_hit_all_intersections_positive_t() {
        //The hit is first item returned, when all intersections have positive t
        let s = spheres::sphere();
        let i1 = intersection(1.0, s.clone());
        let i2 = intersection(2.0, s.clone());
        let xs = intersection_list(vec![i2, i1]);
        match hit(xs) {
            Err(e) => println!("test_hit_intersections_positive_t: {}", e),
            Ok(h) => {
                assert_eq!(h.t == 1.0, true);
            }
        }
    }

    #[test]
    fn test_hit_some_intersections_negative_t() {
        //The hit, when some intersections have negative t
        let s = spheres::sphere();
        let i1 = intersection(-1.0, s.clone());
        let i2 = intersection(1.0, s.clone());
        let xs = intersection_list(vec![i2, i1]);
        match hit(xs) {
            Err(e) => println!("test_hit_some_intersections_negative_t: {}", e),
            Ok(h) => {
                assert_eq!(h.t == 1.0, true);
            }
        }
    }

    #[test]
    fn test_hit_all_intersections_negative_t() {
        //The hit, when all intersections have negative t
        let s = spheres::sphere();
        let i1 = intersection(-2.0, s.clone());
        let i2 = intersection(-1.0, s.clone());
        let xs = intersection_list(vec![i2, i1]);
        match hit(xs) {
            Err(e) => assert_eq!(e.to_string() == "No hit", true),
            Ok(_) => {
                println!("test_hit_all_intersections_negative_t",);
            }
        }
    }

    #[test]
    fn test_hit_always_lowest_nonnegative() {
        //The hit is always the lowest nonnegative intersection
        let s = spheres::sphere();
        let i1 = intersection(5.0, s.clone());
        let i2 = intersection(7.0, s.clone());
        let i3 = intersection(-3.0, s.clone());
        let i4 = intersection(2.0, s.clone());
        let xs = intersection_list(vec![i2, i1, i3, i4]);
        match hit(xs) {
            Err(e) => println!("test_hit_some_intersections_negative_t: {}", e),
            Ok(h) => {
                assert_eq!(h.t == 2.0, true);
            }
        }
    }

    #[test]
    fn test_prepare_computations() {
        //Prepare computations
        let p = tuples::point(0.0, 0.0, -5.0);
        let d = tuples::vector(0.0, 0.0, 1.0);
        let s = spheres::sphere();
        let i = intersection(4.0, s);
        let r = rays::ray(p.clone(), d.clone());
        let testp = &tuples::point(0.0, 0.0, -1.0);
        let testv = &tuples::vector(0.0, 0.0, -1.0);
        let comps = prepare_computations(i.clone(), r);
        assert_eq!(comps.t == i.t, true);
        assert_eq!(
            tuples::get_bool_colors_are_equal(
                &comps.object.material.color,
                &i.object.material.color
            ),
            true
        );
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&comps.point, &testp),
            true
        );
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&comps.eyev, &testv.clone()),
            true
        );
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&comps.normalv, &testv),
            true
        );
    }

    #[test]
    fn test_hit_intersection_outside() {
        //The hit, when an intersection occurs on the outside
        let p = tuples::point(0.0, 0.0, -5.0);
        let d = tuples::vector(0.0, 0.0, 1.0);
        let s = spheres::sphere();
        let i = intersection(4.0, s);
        let r = rays::ray(p.clone(), d.clone());
        let comps = prepare_computations(i.clone(), r);
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn test_hit_intersection_inside() {
        //The hit, when an intersection occurs on the inside
        let p = tuples::point(0.0, 0.0, 0.0);
        let d = tuples::vector(0.0, 0.0, 1.0);
        let s = spheres::sphere();
        let i = intersection(1.0, s);
        let r = rays::ray(p.clone(), d.clone());
        let testp = tuples::point(0.0, 0.0, 1.0);
        let testv = tuples::vector(0.0, 0.0, -1.0);
        let comps = prepare_computations(i.clone(), r);
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&comps.point, &testp),
            true
        );
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&comps.eyev, &testv.clone()),
            true
        );
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&comps.normalv, &testv),
            true
        );
        assert_eq!(comps.inside, true);
    }
}
