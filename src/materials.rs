use crate::lights;
use crate::tuples;

pub const MATERIAL_DEFAULT: Material = Material {
    color: tuples::COLOR_WHITE,
    ambient: 0.1,
    diffuse: 0.9,
    specular: 0.9,
    shininess: 200.0,
};

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub color: tuples::Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

pub fn material(
    color: tuples::Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
) -> Material {
    Material {
        color: color,
        ambient: ambient,
        diffuse: diffuse,
        specular: specular,
        shininess: shininess,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let result = lights::lighting(MATERIAL_DEFAULT, light, tuples::POINT_ORIGIN, eyev, normalv);
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
        let result = lights::lighting(MATERIAL_DEFAULT, light, tuples::POINT_ORIGIN, eyev, normalv);
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
        let result = lights::lighting(MATERIAL_DEFAULT, light, tuples::POINT_ORIGIN, eyev, normalv);
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
        let result = lights::lighting(MATERIAL_DEFAULT, light, tuples::POINT_ORIGIN, eyev, normalv);
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
        let result = lights::lighting(MATERIAL_DEFAULT, light, tuples::POINT_ORIGIN, eyev, normalv);
        println!("result {},{},{}", result.red, result.green, result.blue);
        assert_eq!(
            tuples::get_bool_colors_are_equal(&result, &tuples::color(0.1, 0.1, 0.1)),
            true
        );
    }
}
