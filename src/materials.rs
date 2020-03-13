use crate::tuples;

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

pub const MATERIAL_DEFAULT: Material = Material {
    color: tuples::COLOR_BLACK,
    ambient: 0.1,
    diffuse: 0.9,
    specular: 0.9,
    shininess: 200.0,
};

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
}
