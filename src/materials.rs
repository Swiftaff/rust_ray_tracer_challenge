use crate::patterns;
use crate::tuples;

pub const MATERIAL_DEFAULT: Material = Material {
    pattern: None,
    color: tuples::COLOR_WHITE,
    ambient: 0.1,
    diffuse: 0.9,
    specular: 0.9,
    shininess: 200.0,
    reflective: 0.0,
    transparency: 0.0,
    refractive_index: REFRACTIVE_INDEX_VACUUM,
};

pub const REFRACTIVE_INDEX_VACUUM: f64 = 1.0;
pub const REFRACTIVE_INDEX_AIR: f64 = 1.00029;
pub const REFRACTIVE_INDEX_WATER: f64 = 1.333;
pub const REFRACTIVE_INDEX_GLASS: f64 = 1.52;
pub const REFRACTIVE_INDEX_DIAMOND: f64 = 2.417;

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub pattern: Option<patterns::Pattern>,
    pub color: tuples::Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub transparency: f64,
    pub refractive_index: f64,
}

pub fn material(
    pattern: Option<patterns::Pattern>,
    color: tuples::Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
    reflective: f64,
    transparency: f64,
    refractive_index: f64,
) -> Material {
    Material {
        pattern: pattern,
        color: color,
        ambient: ambient,
        diffuse: diffuse,
        specular: specular,
        shininess: shininess,
        reflective: reflective,
        transparency: transparency,
        refractive_index: refractive_index,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lights;
    use crate::spheres;

    #[test]
    fn test_default_material() {
        //The default material
        let a = MATERIAL_DEFAULT;
        assert_eq!(
            tuples::get_bool_colors_are_equal(&a.color, &tuples::color(1.0, 1.0, 1.0)),
            true
        );
        assert_eq!(a.ambient, 0.1);
        assert_eq!(a.diffuse, 0.9);
        assert_eq!(a.specular, 0.9);
        assert_eq!(a.shininess, 200.0);
    }

    #[test]
    fn test_lighting_surface_eye_light() {
        //Lighting with the eye between the light and the surface
        let eyev = tuples::vector(0.0, 0.0, -1.0);
        let normalv = tuples::vector(0.0, 0.0, -1.0);
        let light = lights::light_point(tuples::point(0.0, 0.0, -10.0), tuples::COLOR_WHITE);
        let s = spheres::sphere();
        let result = lights::lighting(
            MATERIAL_DEFAULT,
            s,
            light,
            tuples::POINT_ORIGIN,
            eyev,
            normalv,
            false,
        );
        assert_eq!(
            tuples::get_bool_colors_are_equal(&result, &tuples::color(1.9, 1.9, 1.9)),
            true
        );
    }

    #[test]
    fn test_lighting_surface_eye_light_eye_offset_45deg() {
        //Lighting with the eye between light and surface, eye offset 45°
        let eyev = tuples::vector(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = tuples::vector(0.0, 0.0, -1.0);
        let light = lights::light_point(tuples::point(0.0, 0.0, -10.0), tuples::COLOR_WHITE);
        let s = spheres::sphere();
        let result = lights::lighting(
            MATERIAL_DEFAULT,
            s,
            light,
            tuples::POINT_ORIGIN,
            eyev,
            normalv,
            false,
        );
        assert_eq!(
            tuples::get_bool_colors_are_equal(&result, &tuples::color(1.0, 1.0, 1.0)),
            true
        );
    }

    #[test]
    fn test_lighting_eye_opposite_surface_light_offset_45deg() {
        //Lighting with eye opposite surface, light offset 45°
        let eyev = tuples::vector(0.0, 0.0, -1.0);
        let normalv = tuples::vector(0.0, 0.0, -1.0);
        let light = lights::light_point(tuples::point(0.0, 10.0, -10.0), tuples::COLOR_WHITE);
        let s = spheres::sphere();
        let result = lights::lighting(
            MATERIAL_DEFAULT,
            s,
            light,
            tuples::POINT_ORIGIN,
            eyev,
            normalv,
            false,
        );
        assert_eq!(
            tuples::get_bool_colors_are_equal(&result, &tuples::color(0.7364, 0.7364, 0.7364)),
            true
        );
    }

    #[test]
    fn test_lighting_eye_in_path_reflection_vector() {
        //Lighting with eye in the path of the reflection vector
        let eyev = tuples::vector(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = tuples::vector(0.0, 0.0, -1.0);
        let light = lights::light_point(tuples::point(0.0, 10.0, -10.0), tuples::COLOR_WHITE);
        let s = spheres::sphere();
        let result = lights::lighting(
            MATERIAL_DEFAULT,
            s,
            light,
            tuples::POINT_ORIGIN,
            eyev,
            normalv,
            false,
        );
        assert_eq!(
            tuples::get_bool_colors_are_equal(&result, &tuples::color(1.6364, 1.6364, 1.6364)),
            true
        );
    }

    #[test]
    fn test_lighting_light_behind_surface() {
        //Lighting with the light behind the surface
        let eyev = tuples::vector(0.0, 0.0, -1.0);
        let normalv = tuples::vector(0.0, 0.0, -1.0);
        let light = lights::light_point(tuples::point(0.0, 0.0, 10.0), tuples::COLOR_WHITE);
        let s = spheres::sphere();
        let result = lights::lighting(
            MATERIAL_DEFAULT,
            s,
            light,
            tuples::POINT_ORIGIN,
            eyev,
            normalv,
            false,
        );
        println!("result {},{},{}", result.red, result.green, result.blue);
        assert_eq!(
            tuples::get_bool_colors_are_equal(&result, &tuples::color(0.1, 0.1, 0.1)),
            true
        );
    }

    #[test]
    fn test_lighting_with_a_pattern_applied() {
        //Lighting with a pattern applied
        let eyev = tuples::vector(0.0, 0.0, -1.0);
        let normalv = tuples::vector(0.0, 0.0, -1.0);
        let light = lights::light_point(tuples::point(0.0, 0.0, -10.0), tuples::COLOR_WHITE);
        let mut m = MATERIAL_DEFAULT;
        m.pattern = Some(patterns::PATTERN_DEFAULT);
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;
        let s = spheres::sphere();
        let c1 = lights::lighting(
            m.clone(),
            s.clone(),
            light.clone(),
            tuples::point(0.9, 0.0, 0.0),
            eyev.clone(),
            normalv.clone(),
            false,
        );
        let c2 = lights::lighting(
            m.clone(),
            s,
            light.clone(),
            tuples::point(1.1, 0.0, 0.0),
            eyev.clone(),
            normalv.clone(),
            false,
        );
        assert_eq!(
            tuples::get_bool_colors_are_equal(&c1, &tuples::COLOR_WHITE),
            true
        );
        assert_eq!(
            tuples::get_bool_colors_are_equal(&c2, &tuples::COLOR_BLACK),
            true
        );
    }

    #[test]
    fn test_refelectivity_for_default_material() {
        //Reflectivity for the default material
        let a = MATERIAL_DEFAULT;
        assert_eq!(tuples::get_bool_numbers_are_equal(a.reflective, 0.0), true);
    }

    #[test]
    fn test_transparency_and_refractive_index_for_default_material() {
        //Transparency and reflective index for default material
        let a = MATERIAL_DEFAULT;
        assert_eq!(
            tuples::get_bool_numbers_are_equal(a.transparency, 0.0),
            true
        );
        assert_eq!(
            tuples::get_bool_numbers_are_equal(a.refractive_index, REFRACTIVE_INDEX_VACUUM),
            true
        );
    }
}
