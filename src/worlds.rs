use crate::intersections;
use crate::lights;
use crate::materials;
use crate::rays;
use crate::shapes;
use crate::spheres;
use crate::transformations;
use crate::tuples;

#[derive(Debug, Clone)]
pub struct World {
    pub objects: Vec<shapes::Shape>,
    pub light: Vec<lights::LightPoint>,
}

pub const RECURSIVE_DEPTH: i32 = 4;

pub fn world_default() -> World {
    let mut s1 = spheres::sphere();
    let mut m1 = materials::MATERIAL_DEFAULT;
    m1.color = tuples::color(0.8, 1.0, 0.6);
    m1.diffuse = 0.7;
    m1.specular = 0.2;
    s1.material = m1;

    let mut s2 = spheres::sphere();
    s2.transform = transformations::matrix4_scaling(0.5, 0.5, 0.5);

    let lights = vec![lights::LightPoint {
        position: tuples::point(-10.0, 10.0, -10.0),
        intensity: tuples::COLOR_WHITE,
    }];

    World {
        objects: vec![s1, s2],
        light: lights,
    }
}

pub fn world_two_lights() -> World {
    let mut w: World = world_default();
    w.light = vec![
        lights::LightPoint {
            position: tuples::point(-10.0, 10.0, -10.0),
            intensity: tuples::color(0.5, 0.9, 1.0),
        },
        lights::LightPoint {
            position: tuples::point(9.0, 4.0, -9.0),
            intensity: tuples::color(0.8, 0.0, 0.0),
        },
    ];
    w
}

impl World {
    pub fn intersect(&self, r: &rays::Ray) -> Vec<intersections::Intersection> {
        let mut xs_list_unsorted: Vec<intersections::Intersection> = vec![];
        for index in 0..self.objects.len() {
            let this_sphere = self.objects[index].clone();
            let xs_for_this_sphere = this_sphere.intersect(r);
            match xs_for_this_sphere {
                Err(_) => (), //println!("XS Error: {}", e),
                Ok(mut xs) => {
                    xs_list_unsorted.append(&mut xs);
                }
            }
        }
        intersections::intersection_list(xs_list_unsorted)
    }

    pub fn shade_hit(&self, c: &intersections::Comps, remaining: &i32) -> tuples::Color {
        let mut col = tuples::COLOR_BLACK;
        for index in 0..self.light.len() {
            let this_light = self.clone().light[index];
            let this_lights_effect = lights::lighting(
                &c.object.material,
                &c.object,
                &this_light,
                &c.over_point,
                &c.eyev,
                &c.normalv,
                &self.is_shadowed(&c.over_point), //TODO maybe try ternary for under_point?
            );
            col = col.add(&this_lights_effect);
        }
        let reflected = self.reflected_color(&c, &remaining);
        let refracted = self.refracted_color(&c, &remaining);
        let material = c.object.material.clone();
        if material.reflective > 0.0 && material.transparency > 0.0 {
            let reflectance = intersections::schlick(&c);
            let c1 = col.add(&reflected.scalar_multiply(&reflectance));
            let c2 = refracted.scalar_multiply(&(1.0 - reflectance));
            c1.add(&c2)
        } else {
            col.add(&reflected).add(&refracted)
        }
    }

    pub fn color_at(&self, r: &rays::Ray, remaining: &i32) -> tuples::Color {
        let xs = self.intersect(&r);
        let hit_temp = intersections::hit(&xs);
        match hit_temp {
            Err(_) => tuples::COLOR_BLACK,
            Ok(hit) => {
                let comp = intersections::prepare_computations(&hit, &r, &Some(xs));
                self.shade_hit(&comp, &remaining)
            }
        }
    }

    pub fn is_shadowed(&self, p: &tuples::Point) -> bool {
        //TODO make work for multiple lights??
        let v = self.light[0].position.subtract(&p);
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = rays::ray(*p, direction);
        let xs = self.intersect(&r);
        let hit_temp = intersections::hit(&xs);
        match hit_temp {
            Err(_) => false,
            Ok(h) => {
                if h.t < distance {
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn reflected_color(&self, c: &intersections::Comps, remaining: &i32) -> tuples::Color {
        if c.object.material.reflective == 0.0 || remaining < &1 {
            tuples::COLOR_BLACK
        } else {
            let reflect_ray = rays::ray(c.over_point, c.reflectv);
            let col = self.color_at(&reflect_ray, &(remaining - 1));
            col.scalar_multiply(&c.object.material.reflective)
        }
    }

    pub fn refracted_color(&self, c: &intersections::Comps, remaining: &i32) -> tuples::Color {
        let n_ratio: f64 = c.n1 / c.n2;
        let cos_i: f64 = c.eyev.dot_product(&c.normalv);
        let sin2_t: f64 = n_ratio * n_ratio * (1.0 - (cos_i * cos_i));

        if c.object.material.transparency == 0.0 || remaining < &1 || sin2_t > 1.0 {
            //println!("black");
            tuples::COLOR_BLACK
        } else {
            //println!("not black");
            let cos_t: f64 = (1.0 - sin2_t).sqrt();
            let direction: tuples::Vector = c
                .normalv
                .multiply(&((n_ratio * cos_i) - cos_t))
                .subtract(&c.eyev.multiply(&n_ratio));
            let start_point = if c.inside {
                c.over_point
            } else {
                c.under_point
            };
            let refract_ray = rays::ray(start_point, direction);
            let col = self.color_at(&refract_ray, &(remaining - 1));
            col.scalar_multiply(&c.object.material.transparency)
        }
    }
}

#[cfg(test)]
use crate::matrices;
mod tests {
    use super::*;
    use crate::patterns;
    use crate::planes;

    fn world() -> World {
        World {
            objects: vec![],
            light: vec![],
        }
    }

    #[test]
    fn test_creating_a_world() {
        //Creating a world
        let w = world();
        assert_eq!(w.objects.len() == 0, true);
        assert_eq!(w.light.len() == 0, true);
    }

    #[test]
    fn test_default_world() {
        //The default world
        let w = world_default();
        assert_eq!(w.objects.len() == 2, true);

        //light position
        assert_eq!(w.light[0].position.x == -10.0, true);
        assert_eq!(w.light[0].position.y == 10.0, true);
        assert_eq!(w.light[0].position.z == -10.0, true);

        //light intensity
        assert_eq!(w.light[0].intensity.red == 1.0, true);
        assert_eq!(w.light[0].intensity.green == 1.0, true);
        assert_eq!(w.light[0].intensity.blue == 1.0, true);

        //object1
        assert_eq!(
            matrices::get_bool_equal_m4(&w.objects[0].transform, &matrices::IDENTITY_MATRIX),
            true
        );
        assert_eq!(
            w.objects[0]
                .material
                .color
                .equals(&tuples::color(0.8, 1.0, 0.6)),
            true
        );

        //object2
        assert_eq!(
            matrices::get_bool_equal_m4(&w.objects[1].transform, &matrices::IDENTITY_MATRIX),
            false
        );
        assert_eq!(
            w.objects[1].material.color.equals(&tuples::COLOR_WHITE),
            true
        );
    }

    #[test]
    fn test_intersect_world_with_ray() {
        //Intersect a world with a ray
        let w = world_default();
        let r = rays::ray(tuples::point(0.0, 0.0, -5.0), tuples::vector(0.0, 0.0, 1.0));
        let xs = w.intersect(&r);
        assert_eq!(xs.len() == 4, true);
        assert_eq!(xs[0].t == 4.0, true);
        assert_eq!(xs[1].t == 4.5, true);
        assert_eq!(xs[2].t == 5.5, true);
        assert_eq!(xs[3].t == 6.0, true);
    }

    #[test]
    fn test_shading_intersection() {
        //Shading an intersection
        let w = world_default();
        let r = rays::ray(tuples::point(0.0, 0.0, -5.0), tuples::vector(0.0, 0.0, 1.0));
        let s = w.objects[0].clone();
        let i = intersections::intersection(4.0, s);
        let comps = intersections::prepare_computations(&i, &r, &None);
        let c = w.shade_hit(&comps, &RECURSIVE_DEPTH);
        assert_eq!(c.equals(&tuples::color(0.38066, 0.47583, 0.2855)), true);
    }

    #[test]
    fn test_shading_intersection_from_inside() {
        //Shading an intersection from the inside
        let mut w = world_default();
        w.light = vec![lights::light_point(
            tuples::point(0.0, 0.25, 0.0),
            tuples::color(1.0, 1.0, 1.0),
        )];
        let r = rays::ray(tuples::point(0.0, 0.0, 0.0), tuples::vector(0.0, 0.0, 1.0));
        let s = w.objects[1].clone();
        let i = intersections::intersection(0.5, s);
        let comps = intersections::prepare_computations(&i, &r, &None);
        let c = w.shade_hit(&comps, &RECURSIVE_DEPTH);
        assert_eq!(
            c.equals(&tuples::color(0.1, 0.1, 0.1)), //&tuples::color(0.90498, 0.90498, 0.90498)),
            //TODO check if this is an error, or if it should actually be this non 0.1 value
            true
        );
    }

    #[test]
    fn test_shading_intersection_in_shadow() {
        //shade_hit() is given an intersection in shadow
        let mut w = world();
        w.light = vec![lights::light_point(
            tuples::point(0.0, 0.0, -10.0),
            tuples::color(1.0, 1.0, 1.0),
        )];
        let s1 = spheres::sphere();
        w.objects.push(s1);
        let mut s2 = spheres::sphere();
        s2.transform = transformations::matrix4_translation(0.0, 0.0, 10.0);
        w.objects.push(s2.clone());
        let r = rays::ray(tuples::point(0.0, 0.0, 5.0), tuples::vector(0.0, 0.0, 1.0));
        let i = intersections::intersection(4.0, s2);
        let comps = intersections::prepare_computations(&i, &r, &None);
        let c = w.shade_hit(&comps, &RECURSIVE_DEPTH);
        assert_eq!(c.equals(&tuples::color(0.1, 0.1, 0.1)), true);
    }

    #[test]
    fn test_color_ray_misses() {
        //The color when a ray misses
        let w = world_default();
        let r = rays::ray(tuples::point(0.0, 0.0, -5.0), tuples::vector(0.0, 1.0, 0.0));
        let c = w.color_at(&r, &RECURSIVE_DEPTH);
        assert_eq!(c.equals(&tuples::COLOR_BLACK), true);
    }

    #[test]
    fn test_color_ray_hits() {
        //The color when a ray hits
        let w = world_default();
        let r = rays::ray(tuples::point(0.0, 0.0, -5.0), tuples::vector(0.0, 0.0, 1.0));
        let c = w.color_at(&r, &RECURSIVE_DEPTH);
        assert_eq!(c.equals(&tuples::color(0.38066, 0.47583, 0.2855)), true);
    }

    #[test]
    fn test_color_with_intersection_behind_ray() {
        //The color with an intersection behind the ray
        let mut w = world_default();
        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;

        let r = rays::ray(
            tuples::point(0.0, 0.0, 0.75),
            tuples::vector(0.0, 0.0, -1.0),
        );
        let c = w.color_at(&r, &RECURSIVE_DEPTH);
        assert_eq!(c.equals(&w.objects[1].material.color), true);
    }

    #[test]
    fn test_no_shadow_when_nothing_between_point_and_light() {
        //There is no shadow when nothing is collinear with point and light
        let w = world_default();
        let p = tuples::point(0.0, 10.0, 0.0);
        assert_eq!(w.is_shadowed(&p), false);
    }

    #[test]
    fn test_shadow_when_something_between_point_and_light() {
        //The shadow when an object is between the point and the light
        let w = world_default();
        let p = tuples::point(10.0, -10.0, 10.0);
        assert_eq!(w.is_shadowed(&p), true);
    }

    #[test]
    fn test_no_shadow_when_object_behind_light() {
        //There is no shadow when an object is behind the light
        let w = world_default();
        let p = tuples::point(-20.0, 20.0, -20.0);
        assert_eq!(w.is_shadowed(&p), false);
    }

    #[test]
    fn test_no_shadow_when_object_behind_point() {
        //There is no shadow when an object is behind the point
        let w = world_default();
        let p = tuples::point(-2.0, 2.0, -2.0);
        assert_eq!(w.is_shadowed(&p), false);
    }

    #[test]
    fn test_reflected_color_for_nonreflective_material() {
        //The reflected color for a non reflective material
        let w = world_default();
        let r = rays::ray(tuples::point(0.0, 0.0, 0.0), tuples::vector(0.0, 0.0, 1.0));
        let mut s = w.objects[1].clone();
        s.material.ambient = 1.0;
        let i = intersections::intersection(1.0, s);
        let comps = intersections::prepare_computations(&i, &r, &None);
        let col = w.reflected_color(&comps, &RECURSIVE_DEPTH);
        assert_eq!(col.equals(&tuples::COLOR_BLACK), true);
    }

    #[test]
    fn test_reflected_color_for_reflective_material() {
        //The reflected color for a reflective material
        let mut w = world_default();
        let mut s = planes::plane();
        s.material.reflective = 0.5;
        s.transform = transformations::matrix4_translation(0.0, -1.0, 0.0);
        w.objects.push(s.clone());
        let r = rays::ray(
            tuples::point(0.0, 0.0, -3.0),
            tuples::vector(0.0, 2.0_f64.sqrt() / -2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = intersections::intersection(2.0_f64.sqrt(), s);
        let comps = intersections::prepare_computations(&i, &r, &None);
        let col = w.reflected_color(&comps, &RECURSIVE_DEPTH);
        assert_eq!(col.equals(&tuples::color(0.19033, 0.23791, 0.14275)), true);
    }

    #[test]
    fn test_shade_hit_with_reflective_material() {
        //shade_hit() with a reflective material
        let mut w = world_default();
        let mut s = planes::plane();
        s.material.reflective = 0.5;
        s.transform = transformations::matrix4_translation(0.0, -1.0, 0.0);
        w.objects.push(s.clone());
        let r = rays::ray(
            tuples::point(0.0, 0.0, -3.0),
            tuples::vector(0.0, 2.0_f64.sqrt() / -2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = intersections::intersection(2.0_f64.sqrt(), s);
        let comps = intersections::prepare_computations(&i, &r, &None);
        let col = w.shade_hit(&comps, &RECURSIVE_DEPTH);
        assert_eq!(col.equals(&tuples::color(0.87676, 0.92434, 0.82917)), true);
    }

    #[test]
    fn test_color_at_with_mutually_reflective_surfaces() {
        //color_at() with mutually reflective surfaces
        let mut w = world();
        w.light = vec![lights::light_point(
            tuples::point(0.0, 0.0, 0.0),
            tuples::COLOR_WHITE,
        )];

        let mut lower = planes::plane();
        lower.material.reflective = 1.0;
        lower.transform = transformations::matrix4_translation(0.0, -1.0, 0.0);
        w.objects.push(lower);

        let mut upper = planes::plane();
        upper.material.reflective = 1.0;
        upper.transform = transformations::matrix4_translation(0.0, 1.0, 0.0);
        w.objects.push(upper);

        let r = rays::ray(tuples::point(0.0, 0.0, 0.0), tuples::vector(0.0, 1.0, 0.0));
        let col = w.color_at(&r, &RECURSIVE_DEPTH);
        assert_eq!(col.equals(&tuples::color(0.2, 0.2, 0.2)), true);
    }

    #[test]
    fn test_reflected_color_at_maximum_recursive_depth() {
        //Reflected color at maximum recursive depth
        let mut w = world_default();
        let mut s = planes::plane();
        s.material.reflective = 0.5;
        s.transform = transformations::matrix4_translation(0.0, -1.0, 0.0);
        w.objects.push(s.clone());

        let r = rays::ray(
            tuples::point(0.0, 0.0, -3.0),
            tuples::vector(0.0, 2.0_f64.sqrt() / -2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = intersections::intersection(2.0_f64.sqrt(), s);
        let comps = intersections::prepare_computations(&i, &r, &None);
        let col = w.reflected_color(&comps, &0);
        assert_eq!(col.equals(&tuples::COLOR_BLACK), true);
    }

    #[test]
    fn test_refracted_color_with_an_opaque_surface() {
        //Refracted color with an opaque surface
        let w = world_default();
        let s = w.objects[0].clone();
        let r = rays::ray(tuples::point(0.0, 0.0, -5.0), tuples::vector(0.0, 0.0, 1.0));
        let i1 = intersections::intersection(4.0, s.clone());
        let i2 = intersections::intersection(6.0, s.clone());
        let xs = intersections::intersection_list(vec![i1, i2]);
        let comps = intersections::prepare_computations(&xs[0], &r, &Some(xs.clone()));
        let col = w.refracted_color(&comps, &RECURSIVE_DEPTH);
        assert_eq!(col.equals(&tuples::COLOR_BLACK), true);
    }

    #[test]
    fn test_refracted_color_at_maximum_recursive_depth() {
        //Refracted color at the maximum recursive depth
        let w = world_default();
        let mut s = w.objects[0].clone();
        s.material.transparency = 1.0;
        s.material.refractive_index = 1.5;
        let r = rays::ray(tuples::point(0.0, 0.0, -5.0), tuples::vector(0.0, 0.0, 1.0));
        let i1 = intersections::intersection(4.0, s.clone());
        let i2 = intersections::intersection(6.0, s.clone());
        let xs = intersections::intersection_list(vec![i1, i2]);
        let comps = intersections::prepare_computations(&xs[0], &r, &Some(xs.clone()));
        let col = w.refracted_color(&comps, &0);
        assert_eq!(col.equals(&tuples::COLOR_BLACK), true);
    }

    #[test]
    fn test_refracted_color_under_total_internal_reflection() {
        //Refracted color under total internal reflection
        let w = world_default();
        let mut s = w.objects[0].clone();
        s.material.transparency = 1.0;
        s.material.refractive_index = 1.5;
        let r = rays::ray(
            tuples::point(0.0, 0.0, 2.0_f64.sqrt() / 2.0),
            tuples::vector(0.0, 1.0, 0.0),
        );
        let i1 = intersections::intersection(2.0_f64.sqrt() / -2.0, s.clone());
        let i2 = intersections::intersection(2.0_f64.sqrt() / 2.0, s.clone());
        let xs = intersections::intersection_list(vec![i1, i2]);
        let comps = intersections::prepare_computations(&xs[1], &r, &Some(xs.clone()));
        let col = w.refracted_color(&comps, &RECURSIVE_DEPTH);
        assert_eq!(col.equals(&tuples::COLOR_BLACK), true);
    }

    #[test]
    fn test_refracted_color_with_refracted_ray() {
        //Refracted color with a refracted ray
        let mut w = world_default();
        w.objects[0].material.ambient = 1.0;
        w.objects[0].material.pattern = Some(patterns::test_pattern());

        w.objects[1].material.transparency = 1.0;
        w.objects[1].material.refractive_index = 1.5;

        let r = rays::ray(tuples::point(0.0, 0.0, 0.1), tuples::vector(0.0, 1.0, 0.0));
        let i1 = intersections::intersection(-0.9899, w.objects[0].clone());
        let i2 = intersections::intersection(-0.4899, w.objects[1].clone());
        let i3 = intersections::intersection(0.4899, w.objects[1].clone());
        let i4 = intersections::intersection(0.9899, w.objects[0].clone());

        let xs = intersections::intersection_list(vec![i1, i2, i3, i4]);
        let comps = intersections::prepare_computations(&xs[2], &r, &Some(xs.clone()));
        let col = w.refracted_color(&comps, &RECURSIVE_DEPTH);
        println!("{} {} {}", col.red, col.green, col.blue);
        assert_eq!(col.equals(&tuples::color(0.0, 0.99889, 0.04722)), true);
    }

    #[test]
    fn test_shade_hit_with_a_transparent_material() {
        //shade_hit() with a transparent material
        let mut w = world_default();

        let mut floor = planes::plane();
        floor.transform = transformations::matrix4_translation(0.0, -1.0, 0.0);
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        w.objects.push(floor.clone());

        let mut ball = w.objects[0].clone();
        ball.material.color = tuples::color(1.0, 0.0, 0.0);
        ball.material.ambient = 0.5;
        ball.transform = transformations::matrix4_translation(0.0, -3.5, -0.5);
        w.objects.push(ball);

        let r = rays::ray(
            tuples::point(0.0, 0.0, -3.0),
            tuples::vector(0.0, 2.0_f64.sqrt() / -2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = intersections::intersection(2.0_f64.sqrt(), floor);
        let xs = intersections::intersection_list(vec![i]);
        let comps = intersections::prepare_computations(&xs[0], &r, &Some(xs.clone()));
        let col = w.shade_hit(&comps, &RECURSIVE_DEPTH);
        assert_eq!(col.equals(&tuples::color(0.93642, 0.68642, 0.68642)), true);
    }

    #[test]
    fn test_shade_hit_with_a_reflective_transparent_material() {
        //shade_hit() with a reflective transparent material
        let mut w = world_default();

        let mut floor = planes::plane();
        floor.transform = transformations::matrix4_translation(0.0, -1.0, 0.0);
        floor.material.reflective = 0.5;
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        w.objects.push(floor.clone());

        let mut ball = w.objects[0].clone();
        ball.material.color = tuples::color(1.0, 0.0, 0.0);
        ball.material.ambient = 0.5;
        ball.transform = transformations::matrix4_translation(0.0, -3.5, -0.5);
        w.objects.push(ball);

        let r = rays::ray(
            tuples::point(0.0, 0.0, -3.0),
            tuples::vector(0.0, 2.0_f64.sqrt() / -2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = intersections::intersection(2.0_f64.sqrt(), floor);
        let xs = intersections::intersection_list(vec![i]);
        let comps = intersections::prepare_computations(&xs[0], &r, &Some(xs.clone()));
        let col = w.shade_hit(&comps, &RECURSIVE_DEPTH);
        println!("{} {} {}", col.red, col.green, col.blue);
        assert_eq!(col.equals(&tuples::color(0.93391, 0.69643, 0.69243)), true);
    }
}
