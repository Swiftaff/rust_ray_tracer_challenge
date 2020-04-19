use crate::intersections;
use crate::rays;
use crate::shapes;
use crate::tuples;

use std::f64::INFINITY;

pub fn cube() -> shapes::Shape {
    shapes::shape(shapes::ShapeType::Cube)
}

pub fn local_intersect(
    s: shapes::Shape,
    ray: rays::Ray,
) -> Result<Vec<intersections::Intersection>, String> {
    let (xtmin, xtmax) = check_axis(ray.origin.x, ray.direction.x);
    let (ytmin, ytmax) = check_axis(ray.origin.y, ray.direction.y);
    let (ztmin, ztmax) = check_axis(ray.origin.z, ray.direction.z);
    let tmin = get_min(xtmin, ytmin, ztmin);
    let tmax = get_min(xtmax, ytmax, ztmax);
    let i1: intersections::Intersection = intersections::intersection(tmin, s.clone());
    let i2: intersections::Intersection = intersections::intersection(tmax, s.clone());
    Ok(intersections::intersection_list(vec![i1, i2]))
}

fn get_min(a: f64, b: f64, c: f64) -> f64 {
    let items: Vec<f64> = vec![a, b, c];
    items.iter().fold(INFINITY, |a, &b| a.min(b))
}

fn check_axis(o: f64, d: f64) -> (f64, f64) {
    let tmin_numerator: f64 = -1.0 - o;
    let tmax_numerator: f64 = 1.0 - o;
    let tmin: f64;
    let tmax: f64;
    //if d.abs() >= tuples::EPSILON {
    tmin = tmin_numerator / d;
    tmax = tmax_numerator / d;
    //} else {
    //    tmin = tmin_numerator * INFINITY;
    //    tmax = tmax_numerator * INFINITY;
    //}
    if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    }
}

pub fn local_normal_at(local_point: tuples::Point) -> tuples::Vector {
    tuples::tuple_subtract(&local_point, &tuples::POINT_ORIGIN)
}

/*
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_ray_intersects_a_cube() {
        //A ray intersects a cube
        let test_rays: [[f64; 8]; 7] = [
            [5.0, 0.5, 0.0, -1.0, 0.0, 0.0, 4.0, 6.0],
            [-5.0, 0.5, 0.0, 1.0, 0.0, 0.0, 4.0, 6.0],
            [0.5, 5.0, 0.0, 0.0, -1.0, 0.0, 4.0, 6.0],
            [0.5, -5.0, 0.0, 0.0, 1.0, 0.0, 4.0, 6.0],
            [0.5, 0.0, 5.0, 0.0, 0.0, -1.0, 4.0, 6.0],
            [0.5, 0.0, -5.0, 0.0, 0.0, 1.0, 4.0, 6.0],
            [0.0, 0.5, 0.0, 0.0, 0.0, 1.0, -1.0, 1.0],
        ];
        let c = cube();
        for index in 0..test_rays.clone().len() {
            let r = rays::ray(
                tuples::point(
                    test_rays[index][0],
                    test_rays[index][1],
                    test_rays[index][2],
                ),
                tuples::vector(
                    test_rays[index][3],
                    test_rays[index][4],
                    test_rays[index][5],
                ),
            );
            let xs_result = local_intersect(c.clone(), r);
            match xs_result {
                Ok(xs) => {
                    println!(
                        "{}={}, {}={}",
                        xs[0].t, test_rays[index][6], xs[1].t, test_rays[index][7]
                    );
                    assert_eq!(xs.len(), 2);
                    assert_eq!(
                        tuples::get_bool_numbers_are_equal(xs[0].t, test_rays[index][6]),
                        true
                    );
                    assert_eq!(
                        tuples::get_bool_numbers_are_equal(xs[1].t, test_rays[index][7]),
                        true
                    );
                }
                Err(_) => {
                    println!("Not possible in this test");
                    assert_eq!(true, false);
                }
            }
        }
    }
}
*/
