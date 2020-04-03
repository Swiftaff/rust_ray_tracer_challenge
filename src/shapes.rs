use std::f64::consts::PI;
use uuid::Uuid;

use crate::intersections;
use crate::materials;
use crate::matrices;
use crate::rays;
use crate::spheres;
use crate::transformations;
use crate::tuples;

#[derive(Debug, Clone)]
pub struct Shape {
    pub id: String,
    pub transform: matrices::Matrix4,
    pub material: materials::Material,
    pub shapeType: ShapeType,
}

#[derive(Debug, Clone, Copy)]
pub enum ShapeType {
    ShapeSphere,
    ShapeTest,
}

pub fn shape(shapeType: ShapeType) -> Shape {
    Shape {
        id: format!("{}", Uuid::new_v4()),
        transform: matrices::IDENTITY_MATRIX,
        material: materials::MATERIAL_DEFAULT,
        shapeType: shapeType,
    }
}

pub fn set_transform(mut s: Shape, t: matrices::Matrix4) -> Shape {
    s.transform = t;
    s
}

pub fn set_material(mut s: Shape, m: materials::Material) -> Shape {
    s.material = m;
    s
}

pub fn intersect(s: Shape, r: rays::Ray) -> Result<Vec<intersections::Intersection>, String> {
    let r2: rays::Ray = rays::ray_transform(r, matrices::matrix4_inverse(s.transform));
    match s.shapeType {
        ShapeSphere => test_intersect(s, r), //spheres::intersect(s, r),
        ShapeTest => test_intersect(s, r),
    }
}

pub fn test_intersect(s: Shape, r: rays::Ray) -> Result<Vec<intersections::Intersection>, String> {
    //only used by tests:
    //all this function needs to do is return the ray for testing
    //but the calling function outputs a vec[xs] or error string, not a ray
    //so we return a string with the ray values for the assert to compare
    let r2: rays::Ray = rays::ray_transform(r, matrices::matrix4_inverse(s.transform));
    let x: String = format!(
        "p({},{},{},{}),v({},{},{},{})",
        r2.origin.x,
        r2.origin.y,
        r2.origin.z,
        r2.origin.w,
        r2.direction.x,
        r2.direction.y,
        r2.direction.z,
        r2.direction.w
    );
    Err(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shapes_default_transformation() {
        //A shape's default transformation
        let s = shape(ShapeType::ShapeTest);
        assert_eq!(
            matrices::get_bool_equal_m4(s.transform, matrices::IDENTITY_MATRIX),
            true
        );
    }

    #[test]
    fn test_assign_material_to_shape() {
        //A shape may be assigned a material
        let s = shape(ShapeType::ShapeTest);
        assert_eq!(s.material.color.red == tuples::COLOR_WHITE.red, true);
        assert_eq!(s.material.ambient == 0.1, true);
        assert_eq!(s.material.diffuse == 0.9, true);
        assert_eq!(s.material.specular == 0.9, true);
        assert_eq!(s.material.shininess == 200.0, true);
        let mut m = materials::MATERIAL_DEFAULT;
        m.ambient = 1.0;
        let s2 = set_material(s, m);
        assert_eq!(s2.material.ambient == 1.0, true);
    }

    #[test]
    fn test_shapes_have_unique_ids() {
        //Shapes have unique IDs
        let s1 = shape(ShapeType::ShapeTest);
        let s2 = shape(ShapeType::ShapeTest);
        let s3 = shape(ShapeType::ShapeTest);
        let s4 = shape(ShapeType::ShapeTest);
        assert_eq!(s1.id == s2.id, false);
        assert_eq!(s2.id == s3.id, false);
        assert_eq!(s3.id == s4.id, false);
        assert_eq!(s4.id == s1.id, false);
    }

    #[test]
    fn test_intersecting_a_scaled_shape_with_a_ray() {
        //Intersecting a scaled shape with a ray
        let r = rays::ray(tuples::point(0.0, 0.0, -5.0), tuples::vector(0.0, 0.0, 1.0));
        let mut s = shape(ShapeType::ShapeTest);
        s = set_transform(s, transformations::matrix4_scaling(2.0, 2.0, 2.0));
        let expected_error = intersect(s, r);
        match expected_error {
            Ok(o) => println!(
                "not possible to get OK, don't bother asserting just get this pretty message!"
            ),
            Err(e) => assert_eq!(e, "p(0,0,-2.5,1),v(0,0,0.5,0)"),
        }
    }

    #[test]
    fn test_intersecting_a_translated_shape_with_a_ray() {
        //Intersecting a translated shape with a ray
        let r = rays::ray(tuples::point(0.0, 0.0, -5.0), tuples::vector(0.0, 0.0, 1.0));
        let mut s = shape(ShapeType::ShapeTest);
        s = set_transform(s, transformations::matrix4_translation(5.0, 0.0, 0.0));
        let expected_error = intersect(s, r);
        match expected_error {
            Ok(o) => println!(
                "not possible to get OK, don't bother asserting just get this pretty message!"
            ),
            Err(e) => assert_eq!(e, "p(-5,0,-5,1),v(0,0,1,0)"),
        }
    }
}
