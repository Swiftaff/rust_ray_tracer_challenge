use std::f64;

const EPSILON: f64 = 0.00001;

pub struct Tuple {
  x: f64,
  y: f64,
  z: f64,
  w: u32,
}
type Vector = Tuple;
type Point = Tuple;

pub struct Color {
    red: f64,
    green: f64,
    blue: f64
}

pub struct Projectile {
    position: Point,
    velocity: Vector
}

pub struct Environment {
    gravity: Vector,
    wind: Vector
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
    } else {
        t
    }
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



#[cfg(test)]
mod tests {
  use super::*;

    #[test]
    fn test_tuple_is_a_point() {
        //A tuple with w=1.0 is a point
        let a = tuple(4.3, -4.2, 3.1, 1);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1);
    }

    #[test]
    fn test_tuple_is_a_vector() {
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
        let p1 = point(3.0,-2.0,5.0);
        let p2 = point(-2.0,3.0,1.0);
        let a = tuple_add(&p1,&p2);
        assert_eq!(get_bool_tuples_are_equal(&a,&p1),true);
    }
}
