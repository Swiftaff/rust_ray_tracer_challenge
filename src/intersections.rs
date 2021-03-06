use std::cmp::Ordering;

use crate::materials;
use crate::rays;
use crate::shapes;
use crate::tuples;

pub fn comp_default(shape_type: &shapes::ShapeType) -> Comps {
    Comps {
        t: 0.0,
        object: shapes::shape(*shape_type),
        point: tuples::point(0.0, 0.0, 0.0),
        over_point: tuples::point(0.0, 0.0, 0.0),
        under_point: tuples::point(0.0, 0.0, 0.0),
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
    pub under_point: tuples::Point,
    pub eyev: tuples::Vector,
    pub normalv: tuples::Vector,
    pub reflectv: tuples::Vector,
    pub inside: bool,
    pub n1: f64,
    pub n2: f64,
}

impl Comps {
    pub fn schlick(&self) -> f64 {
        let mut cos = self.eyev.dot_product(&self.normalv);
        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));
            if sin2_t > 1.0 {
                return 1.0;
            };
            cos = (1.0 - sin2_t).sqrt();
        }
        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);
        return r0 + (1.0 - r0) * ((1.0 - cos).powi(5));
    }
}
#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: shapes::Shape,
}

impl Intersection {
    pub fn is_equal_to(&self, i2: &Intersection) -> bool {
        tuples::get_bool_numbers_are_equal(&self.t, &i2.t) && self.object.id == i2.object.id
    }

    pub fn prepare_computations(
        &self,
        r: &rays::Ray,
        xs_option: &Option<Vec<Intersection>>,
    ) -> Comps {
        let mut comps: Comps = comp_default(&self.object.shape_type);
        comps.t = self.t;
        comps.object = self.clone().object;
        comps.point = r.position(comps.t);
        comps.eyev = r.direction.multiply(&-1.0);
        comps.normalv = comps.object.normal_at(&comps.point);
        comps.reflectv = comps.eyev.multiply(&-1.0).reflect(&comps.normalv);
        comps.over_point = comps.point.add(&comps.normalv.multiply(&tuples::EPSILON));
        comps.under_point = comps
            .point
            .subtract(&comps.normalv.multiply(&tuples::EPSILON));
        if comps.normalv.dot_product(&comps.eyev) < 0.0 {
            comps.inside = true;
            comps.normalv = comps.normalv.multiply(&-1.0);
        }
        let mut containers: Vec<shapes::Shape> = Vec::new();

        let xs: Vec<Intersection>;
        match xs_option {
            Some(the_xs) => {
                xs = the_xs.clone();
            }
            None => {
                xs = vec![self.clone()];
            }
        }

        let hit_result = hit(&xs);
        match hit_result {
            Ok(_hit) => {
                for index in 0..xs.clone().len() {
                    let i_eq_hit = xs[index].is_equal_to(&self);
                    if i_eq_hit {
                        if containers.len() == 0 {
                            comps.n1 = materials::REFRACTIVE_INDEX_VACUUM;
                        } else {
                            comps.n1 = containers[containers.len() - 1].material.refractive_index;
                        }
                    }

                    let is_object_already_in_container = containers
                        .iter()
                        .position(|x| x.id == xs[index].clone().object.id);
                    match is_object_already_in_container {
                        Some(existing_object_index) => {
                            containers.remove(existing_object_index);
                        }
                        None => {
                            containers.push(xs[index].clone().object);
                        }
                    }

                    if i_eq_hit {
                        if containers.len() == 0 {
                            comps.n2 = materials::REFRACTIVE_INDEX_VACUUM;
                        } else {
                            comps.n2 = containers[containers.len() - 1].material.refractive_index;
                        }
                        break;
                    }
                }
            }
            Err(_) => {
                comps.n1 = materials::REFRACTIVE_INDEX_VACUUM;
                comps.n2 = materials::REFRACTIVE_INDEX_VACUUM;
            }
        }

        comps
    }
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

pub fn hit(xs: &Vec<Intersection>) -> Result<Intersection, &'static str> {
    let mut the_hit: i32 = -1;
    for index in 0..xs.len() {
        if the_hit == -1 && xs[index].t >= tuples::EPSILON {
            the_hit = index as i32;
        }
    }
    if the_hit == -1 {
        Err("No hit")
    } else {
        Ok(xs[the_hit as usize].clone())
    }
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
        assert_eq!(tuples::get_bool_numbers_are_equal(&i.t, &3.5), true);
        assert_eq!(
            i.object.transform.is_equal_to(&matrices::IDENTITY_MATRIX),
            true
        );
        assert_eq!(
            i.object.material.color.is_equal_to(&tuples::COLOR_WHITE),
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
        assert_eq!(tuples::get_bool_numbers_are_equal(&xs[0].t, &1.0), true);
        assert_eq!(tuples::get_bool_numbers_are_equal(&xs[1].t, &2.0), true);
    }

    #[test]
    fn test_hit_all_intersections_positive_t() {
        //The hit is first item returned, when all intersections have positive t
        let s = spheres::sphere();
        let i1 = intersection(1.0, s.clone());
        let i2 = intersection(2.0, s.clone());
        let xs = intersection_list(vec![i2, i1]);
        match hit(&xs) {
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
        match hit(&xs) {
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
        match hit(&xs) {
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
        match hit(&xs) {
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
        let comps = i.prepare_computations(&r, &None);
        assert_eq!(comps.t == i.t, true);
        assert_eq!(
            comps
                .object
                .material
                .color
                .is_equal_to(&i.object.material.color),
            true
        );
        assert_eq!(comps.point.is_equal_to(&testp), true);
        assert_eq!(comps.eyev.is_equal_to(&testv.clone()), true);
        assert_eq!(comps.normalv.is_equal_to(&testv), true);
    }

    #[test]
    fn test_hit_intersection_outside() {
        //The hit, when an intersection occurs on the outside
        let p = tuples::point(0.0, 0.0, -5.0);
        let d = tuples::vector(0.0, 0.0, 1.0);
        let s = spheres::sphere();
        let i = intersection(4.0, s);
        let r = rays::ray(p.clone(), d.clone());
        let comps = i.prepare_computations(&r, &None);
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
        let comps = i.prepare_computations(&r, &None);
        assert_eq!(comps.point.is_equal_to(&testp), true);
        assert_eq!(comps.eyev.is_equal_to(&testv.clone()), true);
        assert_eq!(comps.normalv.is_equal_to(&testv), true);
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
        let comps = i.prepare_computations(&r, &None);
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
        let comps = i.prepare_computations(&r, &None);
        assert_eq!(
            comps.reflectv.is_equal_to(&tuples::vector(
                0.0,
                2.0_f64.sqrt() / 2.0,
                2.0_f64.sqrt() / 2.0
            )),
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
        for inter in 0..xs.clone().len() {
            let comps = xs[inter].prepare_computations(&r, &Some(xs.clone()));
            assert_eq!(
                tuples::get_bool_numbers_are_equal(&comps.n1, &results[inter][0]),
                true
            );
            assert_eq!(
                tuples::get_bool_numbers_are_equal(&comps.n2, &results[inter][1]),
                true
            );
        }
    }

    #[test]
    fn test_the_under_point_is_offset_below_the_surface() {
        //The under point is offset below the surface
        let mut s = spheres::sphere_glass();
        s.transform = transformations::matrix4_translation(0.0, 0.0, 1.0);
        let r = rays::ray(tuples::point(0.0, 0.0, -5.0), tuples::vector(0.0, 0.0, 1.0));
        let i = intersections::intersection(5.0, s);
        let xs = intersection_list(vec![i.clone()]);
        let comps = i.prepare_computations(&r, &Some(xs));
        assert_eq!(comps.under_point.z > tuples::EPSILON / 2.0, true);
        assert_eq!(comps.point.z < comps.under_point.z, true);
    }

    #[test]
    fn test_schlick_approximation_under_total_internal_reflection() {
        //Schlick approximation under total internal reflection
        let s = spheres::sphere_glass();

        let r = rays::ray(
            tuples::point(0.0, 0.0, 2.0_f64.sqrt() / 2.0),
            tuples::vector(0.0, 1.0, 0.0),
        );
        let i1 = intersections::intersection(2.0_f64.sqrt() / -2.0, s.clone());
        let i2 = intersections::intersection(2.0_f64.sqrt() / 2.0, s);
        let xs = intersections::intersection_list(vec![i1, i2]);
        let comps = xs[1].prepare_computations(&r, &Some(xs.clone()));
        let reflectance = comps.schlick();
        assert_eq!(tuples::get_bool_numbers_are_equal(&reflectance, &1.0), true);
    }

    #[test]
    fn test_schlick_approximation_with_a_perpendicular_viewing_angle() {
        //Schlick approximation with a perpendicular angle
        let s = spheres::sphere_glass();

        let r = rays::ray(tuples::point(0.0, 0.0, 0.0), tuples::vector(0.0, 1.0, 0.0));
        let i1 = intersections::intersection(-1.0, s.clone());
        let i2 = intersections::intersection(1.0, s);
        let xs = intersections::intersection_list(vec![i1, i2]);
        let comps = xs[1].prepare_computations(&r, &Some(xs.clone()));
        let reflectance = comps.schlick();
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&reflectance, &0.04257),
            true
        );
    }

    #[test]
    fn test_schlick_approximation_with_a_small_angle_and_n2_greater_than_n1() {
        //Schlick approximation with a small angle and n1 > n1
        let s = spheres::sphere_glass();

        let r = rays::ray(
            tuples::point(0.0, 0.99, -2.0),
            tuples::vector(0.0, 0.0, 1.0),
        );
        let i = intersections::intersection(1.8589, s.clone());
        let xs = intersections::intersection_list(vec![i]);
        let comps = xs[0].prepare_computations(&r, &Some(xs.clone()));
        let reflectance = comps.schlick();
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&reflectance, &0.4901),
            true
        );
    }
}
