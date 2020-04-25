use uuid::Uuid;

use crate::cubes;
use crate::intersections;
use crate::materials;
use crate::matrices;
use crate::planes;
use crate::rays;
use crate::spheres;
use crate::tuples;

#[derive(Debug, Clone)]
pub struct Shape {
    pub id: String,
    pub transform: matrices::Matrix4,
    pub material: materials::Material,
    pub shape_type: ShapeType,
}

#[derive(Debug, Clone, Copy)]
pub enum ShapeType {
    Cube,
    Plane,
    ShapeTest,
    Sphere,
}

pub fn shape(shape_type: ShapeType) -> Shape {
    Shape {
        id: format!("{}", Uuid::new_v4()),
        transform: matrices::IDENTITY_MATRIX,
        material: materials::MATERIAL_DEFAULT,
        shape_type: shape_type,
    }
}

impl Shape {
    pub fn intersect(&self, r: &rays::Ray) -> Result<Vec<intersections::Intersection>, String> {
        let local_r: rays::Ray = r.transform(self.transform.inverse());
        match self.shape_type {
            ShapeType::Cube => cubes::local_intersect(&self, &local_r),
            ShapeType::Plane => planes::local_intersect(&self, &local_r),
            ShapeType::ShapeTest => test_local_intersect(&local_r),
            ShapeType::Sphere => spheres::local_intersect(&self, &local_r),
        }
    }

    pub fn normal_at(&self, world_point: &tuples::Point) -> tuples::Vector {
        let local_point: tuples::Point = self.transform.inverse().tuple_multiply(&world_point);
        let local_normal = match self.shape_type {
            ShapeType::Cube => cubes::local_normal_at(&local_point),
            ShapeType::Plane => planes::local_normal_at(),
            ShapeType::ShapeTest => test_local_normal_at(&local_point),
            ShapeType::Sphere => spheres::local_normal_at(&local_point),
        };
        let mut world_normal: tuples::Vector = self
            .transform
            .inverse()
            .transpose()
            .tuple_multiply(&local_normal);
        world_normal.w = 0;
        world_normal.normalize()
    }
}

fn test_local_intersect(local_r: &rays::Ray) -> Result<Vec<intersections::Intersection>, String> {
    //only used by tests - via intersect - so can't move to tests area
    //all this function needs to do is return the ray for testing
    //but the calling function outputs a vec[xs] or error string, not a ray
    //so we return a string with the ray values for the assert to compare
    let x: String = format!(
        "p({},{},{},{}),v({},{},{},{})",
        local_r.origin.x,
        local_r.origin.y,
        local_r.origin.z,
        local_r.origin.w,
        local_r.direction.x,
        local_r.direction.y,
        local_r.direction.z,
        local_r.direction.w
    );
    Err(x)
}

fn test_local_normal_at(local_point: &tuples::Point) -> tuples::Vector {
    //only used by tests - via normal_at - so can't move to tests area
    local_point.subtract(&tuples::POINT_ORIGIN)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformations;
    use std::f64::consts::PI;

    #[test]
    fn test_shapes_default_transformation() {
        //A shape's default transformation
        let s = shape(ShapeType::ShapeTest);
        assert_eq!(s.transform.is_equal_to(&matrices::IDENTITY_MATRIX), true);
    }

    #[test]
    fn test_assign_material_to_shape() {
        //A shape may be assigned a material
        let mut s = shape(ShapeType::ShapeTest);
        assert_eq!(s.material.color.red == tuples::COLOR_WHITE.red, true);
        assert_eq!(s.material.ambient == 0.1, true);
        assert_eq!(s.material.diffuse == 0.9, true);
        assert_eq!(s.material.specular == 0.9, true);
        assert_eq!(s.material.shininess == 200.0, true);
        let mut m = materials::MATERIAL_DEFAULT;
        m.ambient = 1.0;
        s.material = m;
        assert_eq!(s.material.ambient == 1.0, true);
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
        s.transform = transformations::matrix4_scaling(2.0, 2.0, 2.0);
        let expected_error = s.intersect(&r);
        match expected_error {
            Ok(_) => {
                println!("Not possible in this test");
                assert_eq!(false, true);
            }
            Err(e) => assert_eq!(e, "p(0,0,-2.5,1),v(0,0,0.5,0)"),
        }
    }

    #[test]
    fn test_intersecting_a_translated_shape_with_a_ray() {
        //Intersecting a translated shape with a ray
        let r = rays::ray(tuples::point(0.0, 0.0, -5.0), tuples::vector(0.0, 0.0, 1.0));
        let mut s = shape(ShapeType::ShapeTest);
        s.transform = transformations::matrix4_translation(5.0, 0.0, 0.0);
        let expected_error = s.intersect(&r);
        match expected_error {
            Ok(_) => {
                println!("Not possible in this test");
                assert_eq!(false, true);
            }
            Err(e) => assert_eq!(e, "p(-5,0,-5,1),v(0,0,1,0)"),
        }
    }

    #[test]
    fn test_computing_the_normal_on_a_translated_shape() {
        //Computing the normal on a translated shape
        let mut s = shape(ShapeType::ShapeTest);
        s.transform = transformations::matrix4_translation(0.0, 1.0, 0.0);
        let n = s.normal_at(&tuples::point(0.0, 1.70711, -0.70711));
        assert_eq!(n.is_equal_to(&tuples::vector(0.0, 0.70711, -0.70711)), true)
    }

    #[test]
    fn test_computing_the_normal_on_a_transformed_shape() {
        //Computing the normal on a transformed shape
        let mut s = shape(ShapeType::ShapeTest);
        let scaling = transformations::matrix4_scaling(1.0, 0.5, 1.0);
        let rot_z = transformations::matrix4_rotation_z_rad(PI / 5.0);
        let m = transformations::matrix4_transform_chain(&([rot_z, scaling].to_vec()));
        s.transform = m;
        let n = s.normal_at(&tuples::point(0.0, 2.0_f64.sqrt(), -2.0_f64.sqrt()));
        println!("v({},{},{},{})", n.x, n.y, n.z, n.w,);
        assert_eq!(n.is_equal_to(&tuples::vector(0.0, 0.97014, -0.24254)), true)
    }
}
