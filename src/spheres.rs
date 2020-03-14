use uuid::Uuid;

use crate::materials;
use crate::matrices;
use crate::rays;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub id: String,
    pub transform: matrices::Matrix4,
    pub material: materials::Material,
}
pub fn sphere() -> Sphere {
    Sphere {
        id: format!("sphere-{}", Uuid::new_v4()),
        transform: matrices::IDENTITY_MATRIX,
        material: materials::MATERIAL_DEFAULT,
    }
}

/*
pub fn intersect(s, r) {
    let r2 = transform(r, inverse(s.transform));
    let xs = [];
    let { a, b, d } = discriminant(s, r2);
    if (d < 0) {
        //misses
    } else {
        //hits
        let t1 = trunc((-b - Math.sqrt(d)) / (2 * a));
        let t2 = trunc((-b + Math.sqrt(d)) / (2 * a));
        let i1 = intersection(t1 < t2 ? t1 : t2, s);
        let i2 = intersection(t1 < t2 ? t2 : t1, s);
        xs = list_intersections([i1, i2]);
    }
    return xs;
}
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spheres_have_unique_ids() {
        //Spheres have unique IDs
        let s1 = sphere();
        let s2 = sphere();
        let s3 = sphere();
        let s4 = sphere();
        assert_eq!(s1.id == s2.id, false);
        assert_eq!(s2.id == s3.id, false);
        assert_eq!(s3.id == s4.id, false);
        assert_eq!(s4.id == s1.id, false);
    }

    /*
    #[test]
    fn test_ray_intersects_sphere_at_two_points() {
        //A ray intersects a sphere at two points
        let r = rays::ray(tuples::point(0, 0, -5), tuples::vector(0, 0, 1));
        let s = sphere();
        let xs = intersect(s, r);
        assert_eq!(xs.len() == 2, true);
        assert_eq!(xs[0].t == 4, true);
        assert_eq!(xs[1].t == 6, true);
    }*/
}
