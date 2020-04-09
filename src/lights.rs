use std::f64;

use crate::materials;
use crate::patterns;
use crate::shapes;
use crate::tuples;

#[derive(Debug, Copy, Clone)]
pub struct LightPoint {
    pub position: tuples::Point,
    pub intensity: tuples::Color,
}

pub fn light_point(position: tuples::Point, intensity: tuples::Color) -> LightPoint {
    LightPoint {
        position: position,
        intensity: intensity,
    }
}

pub fn lighting(
    material: materials::Material,
    shape: shapes::Shape,
    light: LightPoint,
    point: tuples::Point,
    eyev: tuples::Point,
    normalv: tuples::Vector,
    in_shadow: bool,
) -> tuples::Color {
    let mut diffuse: tuples::Color = tuples::COLOR_BLACK;
    let mut specular: tuples::Color = tuples::COLOR_BLACK;
    let reflectv: tuples::Vector;
    let reflect_dot_eye: f64;

    let mut _col = tuples::COLOR_WHITE;
    match material.pattern {
        Some(p) => _col = patterns::pattern_at_shape(p, shape, point.clone()),
        None => _col = material.color,
    }

    let effective_color: tuples::Color = tuples::colors_multiply(&_col, &light.intensity);
    let lightv: tuples::Vector =
        tuples::vector_normalize(&tuples::tuple_subtract(&light.position, &point));
    let ambient: tuples::Color = tuples::colors_scalar_multiply(&effective_color, material.ambient);
    let light_dot_normal: f64 = tuples::vector_dot_product(&lightv, &normalv);

    if light_dot_normal >= 0.0 {
        diffuse = tuples::colors_scalar_multiply(
            &tuples::colors_scalar_multiply(&effective_color, material.diffuse),
            light_dot_normal,
        );
        reflectv = tuples::tuple_reflect(&tuples::tuple_multiply(lightv, -1.0), &normalv);
        reflect_dot_eye = tuples::vector_dot_product(&reflectv, &eyev);
        if reflect_dot_eye > 0.0 {
            let factor: f64 = reflect_dot_eye.powf(material.shininess);
            specular = tuples::colors_scalar_multiply(&light.intensity, material.specular * factor);
        }
    }
    if in_shadow == true {
        ambient
    } else {
        tuples::colors_add(&tuples::colors_add(&ambient, &diffuse), &specular)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spheres;

    #[test]
    fn test_light_point_has_position_intensity() {
        //A point light has a position and intensity
        let intensity = tuples::COLOR_WHITE;
        let position = tuples::POINT_ORIGIN;
        let light = light_point(position, intensity);
        assert_eq!(
            tuples::get_bool_colors_are_equal(&light.intensity, &intensity),
            true
        );
        assert_eq!(
            tuples::get_bool_tuples_are_equal(&light.position, &position),
            true
        );
    }

    #[test]
    fn test_lighting_with_surface_in_shadow() {
        //Lighting with the surface in shadow
        let eyev = tuples::vector(0.0, 0.0, -1.0);
        let normalv = tuples::vector(0.0, 0.0, -1.0);

        let position = tuples::point(0.0, 0.0, -10.0);
        let intensity = tuples::COLOR_WHITE;
        let light = light_point(position, intensity);
        let in_shadow = true;
        let s = spheres::sphere();
        let col = lighting(
            materials::MATERIAL_DEFAULT,
            s,
            light,
            position,
            eyev,
            normalv,
            in_shadow,
        );
        assert_eq!(
            tuples::get_bool_colors_are_equal(&col, &tuples::color(0.1, 0.1, 0.1)),
            true
        );
    }
}
