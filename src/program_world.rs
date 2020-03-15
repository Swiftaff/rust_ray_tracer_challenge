use chrono::prelude::*;
use std::f64::consts::PI;
use std::fs;
use std::time::Instant;

use crate::camera;
use crate::canvas;
use crate::intersections;
use crate::lights;
use crate::materials;
use crate::matrices;
use crate::rays;
use crate::spheres;
use crate::transformations;
use crate::tuples;
use crate::worlds;

pub fn world_main() {
    println!("world - 6 objects!");
    let start1 = Instant::now();

    let mut w = worlds::world_default();
    w.objects = vec![
        shape_floor(),
        shape_wall_left(),
        shape_wall_right(),
        shape_sphere_middle(),
        shape_sphere_right(),
        shape_sphere_left(),
    ];
    w.light = vec![lights::light_point(
        tuples::point(-10.0, 10.0, -10.0),
        tuples::color(1.0, 1.0, 1.0),
    )];

    let mut c = camera::camera(100, 50, PI / 3.0);
    let from = tuples::point(0.0, 1.5, -5.0);
    let to = tuples::point(0.0, 1.0, 0.0);
    let up = tuples::vector(0.0, 1.0, 0.0);
    c.transform = transformations::view_transform(from, to, up);
    let image = camera::render(c, w);
    let duration1 = start1.elapsed();
    println!("Time to calculate data: {:?}", duration1);

    let start2 = Instant::now();
    let data = canvas::ppm_get(image);
    let duration2 = start2.elapsed();
    println!("Time to generate file data: {:?}", duration2);

    let start3 = Instant::now();
    let f = save(data);
    let _f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem saving the file: {:?}", error),
    };
    let duration3 = start3.elapsed();
    println!("Time to save file: {:?}", duration3);
}

fn save(string: String) -> std::io::Result<()> {
    let utc = Utc::now();
    let d = utc.format("%Y-%m-%d-%H-%M").to_string();
    fs::write(format!("images/world_6_objects{}.ppm", d), string)?;
    Ok(())
}

pub fn material_floor() -> materials::Material {
    let mut mat = materials::MATERIAL_DEFAULT;
    mat.color = tuples::color(1.0, 0.9, 0.9);
    mat.specular = 0.0;
    mat
}

pub fn shape_floor() -> spheres::Sphere {
    let mut shape = spheres::sphere();
    shape.transform = transformations::matrix4_scaling(10.0, 0.01, 10.0);
    shape.material = material_floor();
    shape
}

pub fn shape_wall_left() -> spheres::Sphere {
    let mut shape = spheres::sphere();
    let t1 = transformations::matrix4_translation(0.0, 0.0, 5.0);
    let t2 = transformations::matrix4_rotation_y_rad(-PI / 4.0);
    let t3 = transformations::matrix4_rotation_x_rad(PI / 2.0);
    let t4 = transformations::matrix4_scaling(10.0, 0.01, 10.0);
    shape.transform = transformations::matrix4_transform_chain(vec![t4, t3, t2, t1]);
    shape.material = material_floor();
    shape
}

pub fn shape_wall_right() -> spheres::Sphere {
    let mut shape = spheres::sphere();
    let t1 = transformations::matrix4_translation(0.0, 0.0, 5.0);
    let t2 = transformations::matrix4_rotation_y_rad(PI / 4.0);
    let t3 = transformations::matrix4_rotation_x_rad(PI / 2.0);
    let t4 = transformations::matrix4_scaling(10.0, 0.01, 10.0);
    shape.transform = transformations::matrix4_transform_chain(vec![t4, t3, t2, t1]);
    shape.material = material_floor();
    shape
}

pub fn shape_sphere_middle() -> spheres::Sphere {
    let mut shape = spheres::sphere();
    shape.transform = transformations::matrix4_translation(-0.5, 1.0, 0.5);

    let mut mat = materials::MATERIAL_DEFAULT;
    mat.color = tuples::color(0.1, 1.0, 0.5);
    mat.diffuse = 0.7;
    mat.specular = 0.3;

    shape.material = mat;
    shape
}

pub fn shape_sphere_right() -> spheres::Sphere {
    let mut shape = spheres::sphere();
    let t1 = transformations::matrix4_translation(1.5, 0.5, -0.5);
    let t2 = transformations::matrix4_scaling(0.5, 0.5, 0.5);
    shape.transform = transformations::matrix4_transform_chain(vec![t2, t1]);

    let mut mat = materials::MATERIAL_DEFAULT;
    mat.color = tuples::color(0.5, 1.0, 0.1);
    mat.diffuse = 0.7;
    mat.specular = 0.3;

    shape.material = mat;
    shape
}

pub fn shape_sphere_left() -> spheres::Sphere {
    let mut shape = spheres::sphere();
    let t1 = transformations::matrix4_translation(-1.5, 0.33, -0.75);
    let t2 = transformations::matrix4_scaling(0.33, 0.33, 0.33);
    shape.transform = transformations::matrix4_transform_chain(vec![t2, t1]);

    let mut mat = materials::MATERIAL_DEFAULT;
    mat.color = tuples::color(1.0, 0.8, 0.1);
    mat.diffuse = 0.7;
    mat.specular = 0.3;

    shape.material = mat;
    shape
}
