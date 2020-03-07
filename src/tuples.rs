use std::f64;

const EPSILON: f64 = 0.00001;

#[derive(Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: u32,
}
pub type Vector = Tuple;
pub type Point = Tuple;

#[derive(Debug)]
pub struct Color {
    #[allow(dead_code)]
    pub red: f64,
    #[allow(dead_code)]
    pub green: f64,
    #[allow(dead_code)]
    pub blue: f64
}

#[derive(Debug)]
pub struct Projectile {
    pub position: Point,
    pub velocity: Vector
}

#[derive(Debug)]
pub struct Environment {
    pub gravity: Vector,
    pub wind: Vector
}

pub fn tuple(x:f64,y:f64,z:f64,w:u32) -> Tuple {
    Tuple {x:x,y:y,z:z,w:w}
}

pub fn point(x:f64,y:f64,z:f64) -> Point {
    Point {x:x, y:y, z:z, w:1}
}

pub fn vector(x:f64,y:f64,z:f64) -> Vector {
    Vector {x:x, y:y, z:z, w:0}
}

pub fn color(red:f64,green:f64,blue:f64) -> Color {
    Color {red:red, green:green, blue:blue}
}

pub fn projectile(position: Point, velocity: Vector) -> Projectile {
    Projectile {position:position, velocity:velocity}
}

pub fn environment(gravity: Vector, wind: Vector) -> Environment {
    Environment {gravity:gravity, wind:wind}
}

pub fn tuple_add(a: &Tuple, b: &Tuple) -> Tuple {
    let t = tuple(a.x+b.x, a.y+b.y, a.z+b.z, a.w+b.w);
    if t.w > 1 {
        println!("tuple_add: can't add two points!");
        tuple(a.x, a.y, a.z, a.w)
        //TODO - create an error?
    } else {
        t
    }
}

pub fn tuple_subtract(a: &Tuple, b: &Tuple) -> Tuple {
    if a.w < b.w {
        println!("tuple_subtract: can't subtract a point from a vector");
        tuple(a.x, a.y, a.z, a.w)
        //TODO - create an error?
    } else {
        let w = a.w - b.w;
        tuple(a.x - b.x, a.y - b.y, a.z - b.z, w)
    }
}

pub fn vector_negate(a: &Tuple) -> Tuple {
    let t = tuple(-a.x, -a.y, -a.z, a.w);
    if t.w == 1 {
        println!("tuple_negate: can't negate a point");
        tuple(a.x, a.y, a.z, a.w)
        //TODO - create an error?
    } else {
        t
    }
}

pub fn tuple_scalar_multiply(a: &Tuple, s: f64) -> Tuple {
    let x = a.x * s;
    let y = a.y * s;
    let z = a.z * s;
    tuple(x,y,z,a.w)
}

pub fn tuple_scalar_divide(a: &Tuple, s: f64) -> Tuple {
    let x = a.x / s;
    let y = a.y / s;
    let z = a.z / s;
    tuple(x,y,z,a.w)
}

pub fn tuple_reflect(v: &Tuple, normal: &Tuple) -> Tuple {
    let dp = vector_dot_product(&v, &normal);
    let mult1 = tuple_scalar_multiply(&normal, 2.0);
    let mult2 = tuple_scalar_multiply(&mult1, dp);
    tuple_subtract(&v, &mult2)
}

pub fn vector_magnitude(a: &Tuple) -> f64 {
    let x = a.x * a.x;
    let y = a.y *a.y;
    let z = a.z * a.z;
    let w = a.w * a.w;
    (x+y+z+f64::from(w)).sqrt()
}

pub fn vector_normalize(a: &Tuple) -> Tuple {
    let m = vector_magnitude(a);

    let t = tuple(a.x / m, a.y /m, a.z / m, a.w);
    if t.w == 1 {
        println!("vector_normalize: can't normalize a point");
        tuple(a.x, a.y, a.z, a.w)
        //TODO - create an error?
    } else {
        t
    }
}

pub fn vector_dot_product(a: &Tuple, b: &Tuple) -> f64 {
    if a.w == 1 || b.w == 1 {
        println!("vector_dot_product: can only dotproduct two vectors");
        0.0
        //TODO - create an error?
    } else {
        a.x * b.x + a.y * b.y + a.z * b.z
    }
}

pub fn vector_cross_product(a: &Tuple, b: &Tuple) -> Tuple {
    if a.w == 1 || b.w == 1 {
        println!("vector_crossProduct: can only crossproduct two vectors");
        tuple(a.x, a.y, a.z, a.w)
        //TODO - create an error?
    } else {
        vector(a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x)
    }
}

pub fn colors_add(a: &Color, b: &Color) -> Color {
    color(a.red + b.red, a.green + b.green, a.blue + b.blue )
}

pub fn colors_subtract(a: &Color, b: &Color) -> Color {
    color(a.red - b.red, a.green - b.green, a.blue - b.blue )
}

pub fn colors_multiply(a: &Color, b: &Color) -> Color {
    //hadamard_product
    color(a.red * b.red, a.green * b.green, a.blue * b.blue )
}

pub fn colors_scalar_multiply(a: &Color, s: f64) -> Color {
    color(a.red * s, a.green * s, a.blue * s )
}


pub fn tick(env: Environment, proj: Projectile) -> Projectile {
    let v = proj.velocity;
    let p = proj.position;
    let position = tuple_add(&p, &v);
    let env_vector = tuple_add(&env.gravity, &env.wind);
    let velocity = tuple_add(&v, &env_vector);
    //position.x = Math.floor(position.x);
    //position.y = Math.floor(position.y);
    projectile(position, velocity) 
}

pub fn get_bool_numbers_are_equal(a: f64,b: f64) -> bool {
    (a - b).abs() < EPSILON
}

pub fn get_bool_tuples_are_equal(t1: &Tuple, t2: &Tuple) -> bool {
    get_bool_numbers_are_equal(t1.x, t2.x) && get_bool_numbers_are_equal(t1.y, t2.y) && get_bool_numbers_are_equal(t1.z, t2.z) && t1.w == t2.w  
}

pub fn get_bool_colors_are_equal(c1: &Color, c2: &Color) -> bool {
    get_bool_numbers_are_equal(c1.red, c2.red) &&
    get_bool_numbers_are_equal(c1.green, c2.green) &&
    get_bool_numbers_are_equal(c1.blue, c2.blue)
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn test_point_is_a_tuplew1() {
        //A tuple with w=1.0 is a point
        let a = tuple(4.3, -4.2, 3.1, 1);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1);
    }

    #[test]
    fn test_vector_is_a_tuplew0() {
        //A tuple with w=0.0 is a vector
        let a = tuple(4.3, -4.2, 3.1, 0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0);
    }

    #[test]
    fn test_point_is_a_tuple_w1() {
        //point() creates tuples with w=1
        let a = point(4.0, -4.0, 3.0);
        assert_eq!(a.x, 4.0);
        assert_eq!(a.y, -4.0);
        assert_eq!(a.z, 3.0);
        assert_eq!(a.w, 1);
    }

    #[test]
    fn test_vector_is_a_tuple_w0() {
        //test("vector() creates tuples with w=0"
        let a = vector(4.0, -4.0, 3.0);
        assert_eq!(a.x, 4.0);
        assert_eq!(a.y, -4.0);
        assert_eq!(a.z, 3.0);
        assert_eq!(a.w, 0);
    }

    #[test]
    fn test_color_is_rgb() {
        //color() creates (red, green, blue) Color
        let a = color(-0.5, 0.4, 1.7);
        assert_eq!(a.red, -0.5);
        assert_eq!(a.green, 0.4);
        assert_eq!(a.blue, 1.7);
    }

    #[test]
    fn test_projectile_has_position_velocity() {
        //projectile(p,v) creates object with position and velocity
        let p = point(1.0,2.0,3.0);
        let v = vector(4.0,-4.0,3.0);
        let a = projectile(p,v);
        assert_eq!(a.position.x, 1.0);
        assert_eq!(a.position.y, 2.0);
        assert_eq!(a.position.z, 3.0);
        assert_eq!(a.position.w, 1);
        assert_eq!(a.velocity.x, 4.0);
        assert_eq!(a.velocity.y, -4.0);
        assert_eq!(a.velocity.z, 3.0);
        assert_eq!(a.velocity.w, 0);
    }

    #[test]
    fn test_environment_has_gravity_wind() {
        //environment(v,v) creates object with gravity and wind
        let g = vector(1.0,2.0,3.0);
        let w = vector(4.0,-4.0,3.0);
        let a = environment(g,w);
        assert_eq!(a.gravity.x, 1.0);
        assert_eq!(a.gravity.y, 2.0);
        assert_eq!(a.gravity.z, 3.0);
        assert_eq!(a.gravity.w, 0);
        assert_eq!(a.wind.x, 4.0);
        assert_eq!(a.wind.y, -4.0);
        assert_eq!(a.wind.z, 3.0);
        assert_eq!(a.wind.w, 0);
    }

    #[test]
    fn test_tick_creates_projectile() {
        //tick(environment,projectile) creates correct projectile
        let g = vector(0.0,-1.0,0.0);
        let w = vector(1.0,0.0,0.0);
        let e = environment(g,w);

        let p = point(0.0,0.0,0.0);
        let v = vector(1.0,2.0,3.0);
        let proj = projectile(p,v);

        let a = tick(e,proj);
        assert_eq!(a.position.x, 1.0);
        assert_eq!(a.position.y, 2.0);
        assert_eq!(a.position.z, 3.0);
        assert_eq!(a.position.w, 1);
        assert_eq!(a.velocity.x, 2.0);
        assert_eq!(a.velocity.y, 1.0);
        assert_eq!(a.velocity.z, 3.0);
        assert_eq!(a.velocity.w, 0);
    }


    //get_bool_tuples_are_equal

    #[test]
    fn test_tuples_are_equal() {
        //tuples are equal: if exactly the same
        let p1 = point(4.0,-4.0,3.0);
        let p2 = point(4.0,-4.0,3.0);
        let v1 = vector(4.0,-4.0,3.0);
        let v2 = vector(4.0,-4.0,3.0);
        assert_eq!(get_bool_tuples_are_equal(&p1,&p2),true);
        assert_eq!(get_bool_tuples_are_equal(&v1,&v2),true);
    }

    #[test]
    fn test_tuples_are_not_equal() {
        //tuples are NOT equal: if different
        let p1 = point(4.0,-4.0,3.0);
        let p2 = point(3.0,-2.0,-1.0);
        let v1 = vector(4.0,-4.0,3.0);
        let v2 = vector(3.0,-2.0,-1.0);
        assert_eq!(get_bool_tuples_are_equal(&p1,&p2),false);
        assert_eq!(get_bool_tuples_are_equal(&v1,&v2),false);
    }

    #[test]
    fn test_tuples_are_equal_less_than_epsilon() {
        //tuples are equal: if difference is less than EPSILON
        let p1 = point(4.0,-4.0,3.0);
        let p2 = point(4.000001, -4.000001, 3.000001);
        let v1 = vector(4.0,-4.0,3.0);
        let v2 = vector(4.000001, -4.000001, 3.000001);
        assert_eq!(get_bool_tuples_are_equal(&p1,&p2),true);
        assert_eq!(get_bool_tuples_are_equal(&v1,&v2),true);
    }

    #[test]
    fn test_tuples_are_not_equal_more_than_epsilon() {
        //tuples are NOT equal: if difference is more than EPSILON
        let p1 = point(4.0,-4.0,3.0);
        let p2 = point(4.0001, -4.0001, 3.0001);
        let v1 = vector(4.0,-4.0,3.0);
        let v2 = vector(4.0001, -4.0001, 3.0001);
        assert_eq!(get_bool_tuples_are_equal(&p1,&p2),false);
        assert_eq!(get_bool_tuples_are_equal(&v1,&v2),false);
    }

    //tuple_add
    #[test]
    fn test_tuple_add_point_vector_equals_point() {
        //adding two tuples: point + vector = point
        let p = point(3.0,-2.0,5.0);
        let v = vector(-2.0,3.0,1.0);
        let p2 = point(1.0, 1.0, 6.0);
        let a = tuple_add(&p,&v);
        assert_eq!(get_bool_tuples_are_equal(&a,&p2),true);
    }

    #[test]
    fn test_tuple_add_vector_vector_equals_vector() {
        //adding two tuples: vector + vector = vector
        let v1 = vector(3.0,-2.0,5.0);
        let v2 = vector(-2.0,3.0,1.0);
        let v3 = vector(1.0, 1.0, 6.0);
        let a = tuple_add(&v1,&v2);
        assert_eq!(get_bool_tuples_are_equal(&a,&v3),true);
    }

    #[test]
    fn test_tuple_add_vector_point_equals_point() {
        //adding two tuples: vector + point = vector
        let v1 = vector(3.0,-2.0,5.0);
        let p = point(-2.0,3.0,1.0);
        let p2 = point(1.0, 1.0, 6.0);
        let a = tuple_add(&v1,&p);
        assert_eq!(get_bool_tuples_are_equal(&a,&p2),true);
    }

    #[test]
    fn test_tuple_add_point_point_equals_error() {
        //adding two tuples: point + point = first point (and console error)
        //TODO - create an error?
        let p1 = point(3.0,-2.0,5.0);
        let p2 = point(-2.0,3.0,1.0);
        let a = tuple_add(&p1,&p2);
        assert_eq!(get_bool_tuples_are_equal(&a,&p1),true);
    }

    //tuple_subtract
    #[test]
    fn test_tuple_subtract_point_point_equals_vector() {
        //subtract two tuples: point - point = vector
        let p1 = point(3.0,2.0,1.0);
        let p2 = point(5.0,6.0,7.0);
        let v = vector(-2.0, -4.0, -6.0);
        let a = tuple_subtract(&p1,&p2);
        assert_eq!(get_bool_tuples_are_equal(&a,&v),true);
    }

    #[test]
    fn test_tuple_subtract_point_vector_equals_point() {
        //subtract two tuples: point - vector = point
        let p = point(3.0,2.0,1.0);
        let v = vector(5.0,6.0,7.0);
        let p2 = point(-2.0, -4.0, -6.0);
        let a = tuple_subtract(&p,&v);
        assert_eq!(get_bool_tuples_are_equal(&a,&p2),true);
    }

    #[test]
    fn test_tuple_subtract_vector_vector_equals_vector() {
        //subtract two tuples: vector - vector = vector
        let v1 = vector(3.0,2.0,1.0);
        let v2 = vector(5.0,6.0,7.0);
        let v3 = vector(-2.0, -4.0, -6.0);
        let a = tuple_subtract(&v1,&v2);
        assert_eq!(get_bool_tuples_are_equal(&a,&v3),true);
    }

    #[test]
    fn test_tuple_subtract_vector_point_equals_error() {
        //subtract two tuples: vector - point = false (and console error)
        //TODO - create an error?
        let v = vector(3.0,2.0,1.0);
        let p = point(5.0,6.0,7.0);
        let a = tuple_subtract(&v,&p);
        assert_eq!(get_bool_tuples_are_equal(&a,&v),true);
    }

    //vector_negate
    #[test]
    fn test_negate_vector_equals_neg_vector() {
        //negate a vector = -vector
        let v = vector(1.0,-2.0,3.0);
        let v1 = vector_negate(&v);
        let v2 = vector(-1.0,2.0,-3.0);
        assert_eq!(get_bool_tuples_are_equal(&v1,&v2),true);
    }

    #[test]
    fn test_negate_vector_equals_error() {
        //negate a point = orig vector (and console error)
        //TODO - create an error?
        let p = point(1.0,-2.0,3.0);
        let p1 = vector_negate(&p);
        assert_eq!(get_bool_tuples_are_equal(&p1,&p),true);
    }

    //tuple_scalar_multiply
    #[test]
    fn test_multiply_vector_by_scalar() {
        //Multiplying a vector by a scalar
        let v = vector(1.0,-2.0,3.0);
        let v1 = tuple_scalar_multiply(&v, 3.5);
        let v2 = vector(3.5, -7.0, 10.5);
        assert_eq!(get_bool_tuples_are_equal(&v1,&v2),true);
    }

    #[test]
    fn test_multiply_point_by_scalar() {
        //Multiplying a point by a scalar
        let p = point(1.0,-2.0,3.0);
        let p1 = tuple_scalar_multiply(&p, 3.5);
        let p2 = point(3.5, -7.0, 10.5);
        assert_eq!(get_bool_tuples_are_equal(&p1,&p2),true);
    }
    
    #[test]
    fn test_multiply_vector_by_fraction() {
        //Multiplying a vector by a fraction
        let v = vector(1.0,-2.0,3.0);
        let v1 = tuple_scalar_multiply(&v, 0.5);
        let v2 = vector(0.5, -1.0, 1.5);
        assert_eq!(get_bool_tuples_are_equal(&v1,&v2),true);
    }

    #[test]
    fn test_multiply_point_by_fraction() {
        //Multiplying a point by a fraction
        let p = point(1.0,-2.0,3.0);
        let p1 = tuple_scalar_multiply(&p, 0.5);
        let p2 = point(0.5, -1.0, 1.5);
        assert_eq!(get_bool_tuples_are_equal(&p1,&p2),true);
    }
    
    //tuple_divide
    #[test]
    fn test_dividing_vector_by_scalar() {
        //Dividing a vector by a scalar
        let v = vector(1.0,-2.0,3.0);
        let v1 = tuple_scalar_divide(&v, 2.0);
        let v2 = vector(0.5, -1.0, 1.5);
        assert_eq!(get_bool_tuples_are_equal(&v1,&v2),true);
    }

    #[test]
    fn test_dividing_point_by_scalar() {
        //Dividing a vector by a scalar
        let p = point(1.0,-2.0,3.0);
        let p1 = tuple_scalar_divide(&p, 2.0);
        let p2 = point(0.5, -1.0, 1.5);
        assert_eq!(get_bool_tuples_are_equal(&p1,&p2),true);
    }

    //vector_magnitude
    #[test]
    fn test_get_magnitude_of_vector100() {
        //Computing the magnitude ofvector(1, 0, 0)
        let v = vector(1.0,0.0,0.0);
        let a = vector_magnitude(&v);
        assert_eq!(get_bool_numbers_are_equal(a,1.0),true);
    }

    #[test]
    fn test_get_magnitude_of_vector010() {
        //Computing the magnitude of vector(0, 1, 0)
        let v = vector(0.0,1.0,0.0);
        let a = vector_magnitude(&v);
        assert_eq!(get_bool_numbers_are_equal(a,1.0),true);
    }

    #[test]
    fn test_get_magnitude_of_vector001() {
        //Computing the magnitude of vector(0, 0, 1)
        let v = vector(0.0,0.0,1.0);
        let a = vector_magnitude(&v);
        assert_eq!(get_bool_numbers_are_equal(a,1.0),true);
    }

    #[test]
    fn test_get_magnitude_of_vector123() {
        //Computing the magnitude of vector(1,2,3)
        let v = vector(1.0,2.0,3.0);
        let a = vector_magnitude(&v);
        let f = 14.0_f64;
        assert_eq!(get_bool_numbers_are_equal(a,f.sqrt()),true);
    }

    #[test]
    fn test_get_magnitude_of_vector_neg123() {
        //Computing the magnitude of vector(1,2,3)
        let v = vector(-1.0,-2.0,-3.0);
        let a = vector_magnitude(&v);
        let f = 14.0_f64;
        assert_eq!(get_bool_numbers_are_equal(a,f.sqrt()),true);
    }

    //vector_normalize
    #[test]
    fn test_vector_normalize100() {
        //vector_normalize(4, 0, 0) gives vector(1, 0, 0)
        let v = vector(4.0,0.0,0.0);
        let a = vector(1.0,0.0,0.0);
        assert_eq!(get_bool_tuples_are_equal(&vector_normalize(&v),&a),true);
    }

    #[test]
    fn test_vector_normalize123() {
        //vector_normalize(1,2,3) gives vector(1/√14, 2/√14, 3/√14)
        let v = vector(1.0,2.0,3.0);
        let a = vector(1.0/14.0_f64.sqrt(),2.0/14.0_f64.sqrt(),3.0/14.0_f64.sqrt());
        assert_eq!(get_bool_tuples_are_equal(&vector_normalize(&v),&a),true);
    }
    
    #[test]
    fn test_vector_normalize_vec_mag_equals1() {
        //The magnitude of a normalized vector gives 1
        let v = vector(1.0,2.0,3.0);
        let n = vector_normalize(&v);
        let mag = vector_magnitude(&n);
        assert_eq!(get_bool_numbers_are_equal(mag,1.0),true);
    }

    #[test]
    fn test_vector_normalize_point_is_error() {
        //vector_normalize a point = false (and console error)
        let p = point(1.0,2.0,3.0);
        let n = vector_normalize(&p);
        assert_eq!(get_bool_tuples_are_equal(&n,&p),true);
    }

    //vector_dot_product
    #[test]
    fn test_dot_product_two_vectors() {
        //The dot product of two vectors
        let v1 = vector(1.0,2.0,3.0);
        let v2 = vector(2.0,3.0,4.0);
        let a = vector_dot_product(&v1, &v2);
        assert_eq!(get_bool_numbers_are_equal(a,20.0),true);
    }

    #[test]
    fn test_dot_product_point_vector_error() {
        //Can't vector_dot_product points = false (and console error)
        let p1 = point(1.0,2.0,3.0);
        let v1 = vector(2.0,3.0,4.0);
        let a = vector_dot_product(&p1, &v1);
        assert_eq!(get_bool_numbers_are_equal(a,0.0),true);
    }

    #[test]
    fn test_dot_product_point_point_error() {
        //Can't vector_dot_product points = false (and console error)
        let p1 = point(1.0,2.0,3.0);
        let p2 = point(2.0,3.0,4.0);
        let a = vector_dot_product(&p1, &p2);
        assert_eq!(get_bool_numbers_are_equal(a,0.0),true);
    }

    #[test]
    fn test_dot_product_vector_point_error() {
        //Can't vector_dot_product points = false (and console error)
        let v1 = vector(1.0,2.0,3.0);
        let p1 = point(2.0,3.0,4.0);
        let a = vector_dot_product(&v1, &p1);
        assert_eq!(get_bool_numbers_are_equal(a,0.0),true);
    }

    //vector_crossProduct
    #[test]
    fn test_cross_product_vector_vector() {
        //The cross product of two vectors a and b
        let v1 = vector(1.0,2.0,3.0);
        let v2 = vector(2.0,3.0,4.0);
        let v3 = vector_cross_product(&v1, &v2);
        let a = vector(-1.0,2.0,-1.0);
        assert_eq!(get_bool_tuples_are_equal(&v3,&a),true);
    }

    #[test]
    fn test_cross_product_vector_vector_reversed() {
        //The cross product of two vectors b and a
        let v2 = vector(1.0,2.0,3.0);
        let v1 = vector(2.0,3.0,4.0);
        let v3 = vector_cross_product(&v2, &v1);
        let a = vector(-1.0,2.0,-1.0);
        assert_eq!(get_bool_tuples_are_equal(&v3,&a),true);
    }

    #[test]
    fn test_cross_product_point_vector_error() {
        //Can't vector_crossProduct points = false (and console error)
        let p1 = point(1.0,2.0,3.0);
        let v2 = vector(2.0,3.0,4.0);
        let v3 = vector_cross_product(&p1, &v2);
        assert_eq!(get_bool_tuples_are_equal(&v3, &p1),true);
    }

    #[test]
    fn test_cross_product_vector_point_error() {
        //Can't vector_crossProduct points = false (and console error)
        let v1 = vector(1.0,2.0,3.0);
        let p2 = point(2.0,3.0,4.0);
        let v3 = vector_cross_product(&v1, &p2);
        assert_eq!(get_bool_tuples_are_equal(&v3, &v1),true);
    }

    #[test]
    fn test_cross_product_point_point_error() {
        //Can't vector_crossProduct points = false (and console error)
        let p1 = point(1.0,2.0,3.0);
        let p2 = point(2.0,3.0,4.0);
        let v3 = vector_cross_product(&p1, &p2);
        assert_eq!(get_bool_tuples_are_equal(&v3, &p1),true);
    }

    //colors
    #[test]
    fn test_colors_add() {
        //adding two colors
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        let c3 = colors_add(&c1, &c2);
        let a = color(1.6, 0.7, 1.0);
        assert_eq!(get_bool_colors_are_equal(&c3, &a),true);
    }

    #[test]
    fn test_colors_subtract() {
        //subtracting two colors
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        let c3 = colors_subtract(&c1, &c2);
        let a = color(0.2, 0.5, 0.5);
        assert_eq!(get_bool_colors_are_equal(&c3, &a),true);
    }

    
    #[test]
    fn test_colors_scalar_multiply() {
        //multiplying color by a scalar
        let c1 = color(0.2, 0.3, 0.4);
        let c2 = colors_scalar_multiply(&c1, 2.0);
        let a = color(0.4, 0.6, 0.8);
        assert_eq!(get_bool_colors_are_equal(&c2, &a),true);
    }

    #[test]
    fn test_colors_multiply() {
        //multiplying two colors
        let c1 = color(1.0, 0.2, 0.4);
        let c2 = color(0.9, 1.0, 0.1);
        let c3 = colors_multiply(&c1, &c2);
        let a = color(0.9, 0.2, 0.04);
        assert_eq!(get_bool_colors_are_equal(&c3, &a),true);
    }

    #[test]
    fn test_reflect_vector_45degrees() {
        //Reflecting a vector approaching at 45°
        let v = vector(1.0, -1.0, 0.0);
        let n = vector(0.0, 1.0, 0.0);
        let r = tuple_reflect(&v, &n);
        let a = vector(1.0, 1.0, 0.0);
        assert_eq!(get_bool_tuples_are_equal(&r, &a),true);
    }

    #[test]
    fn test_reflect_vector_slanted_surface() {
        //Reflecting a vector off a slanted surface
        let v = vector(0.0, -1.0, 0.0);
        let s = 2.0_f64.sqrt()/2.0;
        let n = vector(s, s, 0.0);
        let r = tuple_reflect(&v, &n);
        let a = vector(1.0, 0.0, 0.0);
        assert_eq!(get_bool_tuples_are_equal(&r, &a),true);
    }
}

