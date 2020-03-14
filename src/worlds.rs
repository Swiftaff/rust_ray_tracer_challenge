use crate::intersections;
use crate::lights;
use crate::materials;
use crate::matrices;
use crate::rays;
use crate::spheres;
use crate::transformations;
use crate::tuples;

#[derive(Debug, Clone)]
pub struct World {
    pub objects: Vec<spheres::Sphere>,
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

pub fn world_intersect(w: World, r: rays::Ray) -> Vec<intersections::Intersection> {
    let mut xs_list_unsorted: Vec<intersections::Intersection> = vec![];
    for index in 0..w.objects.len() {
        let this_sphere = w.objects[index].clone();
        let xs_for_this_sphere = spheres::intersect(this_sphere, *&r);
        match xs_for_this_sphere {
            Err(e) => (), //println!("XS Error: {}", e),
            Ok(mut xs) => {
                xs_list_unsorted.append(&mut xs);
            }
        }
    }
    intersections::intersection_list(xs_list_unsorted)
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
}
