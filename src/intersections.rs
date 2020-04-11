use std::cmp::Ordering;

use crate::rays;
use crate::shapes;

use crate::tuples;

pub fn comp_default(shape_type: &shapes::ShapeType) -> Comps {
    Comps {
        t: 0.0,
        object: shapes::shape(*shape_type),
        point: tuples::point(0.0, 0.0, 0.0),
        over_point: tuples::point(0.0, 0.0, 0.0),
        eyev: tuples::vector(0.0, 0.0, 0.0),
        normalv: tuples::vector(0.0, 0.0, 0.0),
        reflectv: tuples::vector(0.0, 0.0, 0.0),
        inside: false,
        n1: 0.0,
        n2: 0.0,
    }
}

#[derive(Debug, Clone)]
pub struct Comps {
    pub t: f64,
    pub object: shapes::Shape,
    pub point: tuples::Point,
    pub over_point: tuples::Point,
    pub eyev: tuples::Vector,
    pub normalv: tuples::Vector,
    pub reflectv: tuples::Vector,
    pub inside: bool,
    pub n1: f64,
    pub n2: f64,
}

#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: shapes::Shape,
}

pub fn intersection(t: f64, object: shapes::Shape) -> Intersection {
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
    let mut the_hit: i32 = -1;
    for index in 0..xs.len() {
        if the_hit == -1 && xs[index].t >= 0.0 {
            the_hit = index as i32;
        }
    }
    if the_hit == -1 {
        Err("No hit")
    } else {
        Ok(xs[the_hit as usize].clone())
    }
}

pub fn prepare_computations(
    i: Intersection,
    r: rays::Ray,
    xs_option: Option<Vec<Intersection>>,
) -> Comps {
    let mut comps: Comps = comp_default(&i.object.shape_type);
    comps.t = i.clone().t;
    comps.object = i.clone().object;
    comps.point = rays::position(r, comps.t);
    comps.eyev = tuples::tuple_multiply(r.direction, -1.0);
    comps.normalv = shapes::normal_at(comps.clone().object, comps.clone().point);
    comps.reflectv =
        tuples::tuple_reflect(&tuples::tuple_multiply(comps.eyev, -1.0), &comps.normalv);
    comps.over_point = tuples::tuple_add(
        &comps.point,
        &(tuples::tuple_scalar_multiply(&comps.clone().normalv, tuples::EPSILON)),
    );
    if tuples::vector_dot_product(&comps.normalv, &comps.eyev) < 0.0 {
        comps.inside = true;
        comps.normalv = tuples::tuple_multiply(comps.normalv, -1.0);
    }
    let xs: Vec<Intersection>;
    let mut containers: Vec<shapes::Shape> = Vec::new();
    match xs_option {
        Some(the_xs) => {
            xs = the_xs;
        }
        None => {
            xs = vec![i.clone()];
        }
    }
    let hit_result = hit(xs.clone());
    match hit_result {
        Ok(the_hit) => {
            for index in 0..xs.clone().len() {
                if xs[index].object.id == the_hit.object.id {
                    if containers.len() == 0 {
                        comps.n1 = 1.0;
                    } else {
                        comps.n1 = containers[containers.len() - 1].material.refractive_index;
                    }
                }

                let is_object_already_in_container =
                    containers.iter().position(|x| x.id >= i.object.id);
                match is_object_already_in_container {
                    Some(existing_object_index) => {
                        containers.remove(existing_object_index);
                    }
                    None => {
                        containers.push(xs[index].clone().object);
                    }
                }

                if xs[index].object.id == the_hit.object.id {
                    if containers.len() == 0 {
                        comps.n2 = 1.0;
                    } else {
                        comps.n2 = containers.last().unwrap().material.refractive_index;
                    }
                }
            }
        }
        Err(_) => {
            comps.n1 = 1.0;
            comps.n2 = 1.0;
        }
    }
    comps
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intersections;
    use crate::matrices;
    use crate::planes;
    use crate::spheres;
    use crate::transformations;

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
        let comps = prepare_computations(i.clone(), r, None);
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
        let comps = prepare_computations(i.clone(), r, None);
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
        let comps = prepare_computations(i.clone(), r, None);
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

    #[test]
    fn test_hit_should_offset_the_point() {
        //The hit should offset the point
        let p = tuples::point(0.0, 0.0, -5.0);
        let d = tuples::vector(0.0, 0.0, 1.0);
        let mut s = spheres::sphere();
        s.transform = transformations::matrix4_translation(0.0, 0.0, 1.0);
        let i = intersection(5.0, s);
        let r = rays::ray(p, d);
        let comps = prepare_computations(i.clone(), r, None);
        assert_eq!(&comps.over_point.z < &(tuples::EPSILON / -2.0), true);
        assert_eq!(&comps.point.z > &comps.over_point.z, true);
    }

    #[test]
    fn test_precomputing_the_reflection_vector() {
        //Precomputing the reflection vector
        let s = planes::plane();
        let r = rays::ray(
            tuples::point(0.0, 1.0, -1.0),
            tuples::vector(0.0, 2.0_f64.sqrt() / -2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = intersections::intersection(2.0_f64.sqrt(), s);
        let comps = prepare_computations(i, r, None);
        println!(
            "testy {} {} {}",
            comps.reflectv.x, comps.reflectv.y, comps.reflectv.z
        );
        assert_eq!(
            tuples::get_bool_tuples_are_equal(
                &comps.reflectv,
                &tuples::vector(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
            ),
            true
        );
    }

    #[test]
    fn test_finding_n1_and_n2_at_various_intersections() {
        //Finding n1 and n2 at various intersections
        let mut a = spheres::sphere_glass();
        a.transform = transformations::matrix4_scaling(2.0, 2.0, 2.0);
        a.material.refractive_index = 1.5;

        let mut b = spheres::sphere_glass();
        b.transform = transformations::matrix4_translation(0.0, 0.0, -0.25);
        b.material.refractive_index = 2.0;

        let mut c = spheres::sphere_glass();
        c.transform = transformations::matrix4_translation(0.0, 0.0, 0.25);
        c.material.refractive_index = 2.5;

        let r = rays::ray(tuples::point(0.0, 0.0, -4.0), tuples::vector(0.0, 0.0, 1.0));
        let results = [
            [1.0, 1.5],
            [1.5, 2.0],
            [2.0, 2.5],
            [2.5, 2.5],
            [2.5, 1.5],
            [1.5, 1.0],
        ];
        let xs = intersection_list(vec![
            intersection(2.0, a.clone()),
            intersection(2.75, b.clone()),
            intersection(3.25, c.clone()),
            intersection(4.75, b.clone()),
            intersection(5.25, c.clone()),
            intersection(6.0, a.clone()),
        ]);
        for index in 0..xs.clone().len() {
            let comps = prepare_computations(xs[index].clone(), r, Some(xs.clone()));
            println!(
                "index: {}, n1: {}={}, n2: {}={}",
                index, comps.n1, results[index][0], comps.n2, results[index][1]
            );
            assert_eq!(
                tuples::get_bool_numbers_are_equal(comps.n1, results[index][0]),
                true
            );
            assert_eq!(
                tuples::get_bool_numbers_are_equal(comps.n2, results[index][1]),
                true
            );
        }
    }
}
