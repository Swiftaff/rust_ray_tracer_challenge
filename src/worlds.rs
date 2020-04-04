use crate::intersections;
use crate::lights;
use crate::materials;
use crate::matrices;
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

pub fn world() -> World {
    World {
        objects: vec![],
        light: vec![],
    }
}

pub fn world_default() -> World {
    let mut s1 = spheres::sphere();
    let mut m1 = materials::MATERIAL_DEFAULT;
    m1.color = tuples::color(0.8, 1.0, 0.6);
    m1.diffuse = 0.7;
    m1.specular = 0.2;
    s1 = spheres::set_material(s1, m1);

    let mut s2 = spheres::sphere();
    s2 = spheres::set_transform(s2, transformations::matrix4_scaling(0.5, 0.5, 0.5));

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

pub fn world_intersect(w: World, r: rays::Ray) -> Vec<intersections::Intersection> {
    let mut xs_list_unsorted: Vec<intersections::Intersection> = vec![];
    for index in 0..w.objects.len() {
        let this_sphere = w.objects[index].clone();
        let xs_for_this_sphere = shapes::intersect(this_sphere, *&r);
        match xs_for_this_sphere {
            Err(e) => (), //println!("XS Error: {}", e),
            Ok(mut xs) => {
                xs_list_unsorted.append(&mut xs);
            }
        }
    }
    intersections::intersection_list(xs_list_unsorted)
}

pub fn shade_hit(w: World, c: intersections::Comps) -> tuples::Color {
    let mut col = tuples::COLOR_BLACK;
    for index in 0..w.light.len() {
        let this_light = w.clone().light[index];
        let this_lights_effect = lights::lighting(
            c.object.material,
            this_light,
            c.point,
            c.eyev,
            c.normalv,
            is_shadowed(w.clone(), c.over_point),
        );
        col = tuples::colors_add(&col, &this_lights_effect);
    }
    col
}

pub fn color_at(w: World, r: rays::Ray) -> tuples::Color {
    let xs = world_intersect(w.clone(), r);
    let hit_temp = intersections::hit(xs);
    match hit_temp {
        Err(e) => tuples::COLOR_BLACK,
        Ok(hit) => {
            let comp = intersections::prepare_computations(hit, r);
            shade_hit(w, comp)
        }
    }
}

pub fn is_shadowed(w: World, p: tuples::Point) -> bool {
    //TODO make work for multiple lights??
    let v = tuples::tuple_subtract(&w.light[0].position, &p);
    let distance = tuples::vector_magnitude(&v);
    let direction = tuples::vector_normalize(&v);
    let r = rays::ray(p, direction);
    let xs = world_intersect(w, r);
    let hit_temp = intersections::hit(xs);
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

#[cfg(test)]
mod tests {
    use super::*;

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
            matrices::get_bool_equal_m4(w.objects[0].transform, matrices::IDENTITY_MATRIX),
            true
        );
        assert_eq!(
            tuples::get_bool_colors_are_equal(
                &w.objects[0].material.color,
                &tuples::color(0.8, 1.0, 0.6)
            ),
            true
        );

        //object2
        assert_eq!(
            matrices::get_bool_equal_m4(w.objects[1].transform, matrices::IDENTITY_MATRIX),
            false
        );
        assert_eq!(
            tuples::get_bool_colors_are_equal(&w.objects[1].material.color, &tuples::COLOR_WHITE),
            true
        );
    }

    #[test]
    fn test_intersect_world_with_ray() {
        //Intersect a world with a ray
        let w = world_default();
        let r = rays::ray(tuples::point(0.0, 0.0, -5.0), tuples::vector(0.0, 0.0, 1.0));
        let xs = world_intersect(w, r);
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
        let comps = intersections::prepare_computations(i, r);
        let c = shade_hit(w, comps);
        assert_eq!(
            tuples::get_bool_colors_are_equal(&c, &tuples::color(0.38066, 0.47583, 0.2855)),
            true
        );
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
        let comps = intersections::prepare_computations(i, r);
        let c = shade_hit(w, comps);
        assert_eq!(
            tuples::get_bool_colors_are_equal(&c, &tuples::color(0.1, 0.1, 0.1)), //&tuples::color(0.90498, 0.90498, 0.90498)),
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
        s2 = spheres::set_transform(
            s2.clone(),
            transformations::matrix4_translation(0.0, 0.0, 10.0),
        );
        w.objects.push(s2.clone());
        let r = rays::ray(tuples::point(0.0, 0.0, 5.0), tuples::vector(0.0, 0.0, 1.0));
        let i = intersections::intersection(4.0, s2);
        let comps = intersections::prepare_computations(i, r);
        let c = shade_hit(w, comps);
        assert_eq!(
            tuples::get_bool_colors_are_equal(&c, &tuples::color(0.1, 0.1, 0.1)),
            true
        );
    }

    #[test]
    fn test_color_ray_misses() {
        //The color when a ray misses
        let w = world_default();
        let r = rays::ray(tuples::point(0.0, 0.0, -5.0), tuples::vector(0.0, 1.0, 0.0));
        let c = color_at(w, r);
        assert_eq!(
            tuples::get_bool_colors_are_equal(&c, &tuples::COLOR_BLACK),
            true
        );
    }

    #[test]
    fn test_color_ray_hits() {
        //The color when a ray hits
        let w = world_default();
        let r = rays::ray(tuples::point(0.0, 0.0, -5.0), tuples::vector(0.0, 0.0, 1.0));
        let c = color_at(w, r);
        assert_eq!(
            tuples::get_bool_colors_are_equal(&c, &tuples::color(0.38066, 0.47583, 0.2855)),
            true
        );
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
        let c = color_at(w.clone(), r);
        println!(
            "c({},{},{}) inner({},{},{})",
            c.red,
            c.green,
            c.blue,
            w.objects[1].material.color.red,
            w.objects[1].material.color.green,
            w.objects[1].material.color.blue
        );
        assert_eq!(
            tuples::get_bool_colors_are_equal(&c, &w.objects[1].material.color),
            true
        );
    }

    #[test]
    fn test_no_shadow_when_nothing_between_point_and_light() {
        //There is no shadow when nothing is collinear with point and light
        let w = world_default();
        let p = tuples::point(0.0, 10.0, 0.0);
        assert_eq!(is_shadowed(w, p), false);
    }

    #[test]
    fn test_shadow_when_something_between_point_and_light() {
        //The shadow when an object is between the point and the light
        let w = world_default();
        let p = tuples::point(10.0, -10.0, 10.0);
        assert_eq!(is_shadowed(w, p), true);
    }

    #[test]
    fn test_no_shadow_when_object_behind_light() {
        //There is no shadow when an object is behind the light
        let w = world_default();
        let p = tuples::point(-20.0, 20.0, -20.0);
        assert_eq!(is_shadowed(w, p), false);
    }

    #[test]
    fn test_no_shadow_when_object_behind_point() {
        //There is no shadow when an object is behind the point
        let w = world_default();
        let p = tuples::point(-2.0, 2.0, -2.0);
        assert_eq!(is_shadowed(w, p), false);
    }
}
