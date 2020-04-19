use chrono::prelude::*;
//use std::f64::consts::PI;
use std::fs;
use std::time::Instant;

use crate::canvas;
use crate::intersections;
use crate::lights;
use crate::materials;
//use crate::matrices;
use crate::rays;
use crate::shapes;
use crate::spheres;
//use crate::transformations;
use crate::tuples;

pub fn sphere_lighting_main(w: u32, h: u32) {
    println!("sphere lighting");
    let start1 = Instant::now();
    let mut c = canvas::pixel_canvas(w, h, tuples::COLOR_BLACK);
    let ray_origin = tuples::point(0.0, 0.0, -5.0);
    let wall_z: f64 = 10.0;
    let wall_size: f64 = 7.0;
    let pixel_size: f64 = wall_size / w as f64;
    let half: f64 = wall_size / 2.0;
    let mut shape = spheres::sphere();
    let mut mat = materials::MATERIAL_DEFAULT;
    mat.color = tuples::color(1.0, 0.2, 1.0);
    //let m1: matrices::Matrix4 = transformations::matrix4_scaling(0.75, 1.0, 1.0);
    //let m2: matrices::Matrix4 = transformations::matrix4_rotation_z_rad(PI / 8.0);
    //let m3: matrices::Matrix4 = transformations::matrix4_shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    //let t = matrices::matrix4_multiply(m1, m3);
    //shape.transform = t;
    shape.material = mat;

    let light_position = tuples::point(-10.0, 10.0, -10.0);
    let light_color = tuples::color(1.0, 1.0, 1.0);
    let light = lights::light_point(light_position, light_color);
    let mut pc = 5;

    for y in 0..h {
        if y / h > pc {
            println!("...ray tracing: {}%", pc);
            pc = pc + 5;
        }
        let world_y = half - pixel_size * y as f64;
        for x in 0..w {
            let world_x = half - pixel_size * x as f64;
            let position = tuples::point(world_x, world_y, wall_z);
            let r = rays::ray(
                ray_origin,
                tuples::vector_normalize(&tuples::tuple_subtract(&position, &ray_origin)),
            );
            let xs_result = shapes::intersect(&shape, &r);
            match xs_result {
                Err(_) => {} //println!("Error: {}", e),
                Ok(xs) => {
                    let h_result = intersections::hit(&xs);
                    match h_result {
                        Err(_) => {} //println!("Error: {}", e),
                        Ok(h) => {
                            let pnt = rays::position(r, h.t);
                            let nrm = shapes::normal_at(&shape, &pnt);
                            let eye = tuples::tuple_multiply(&r.direction, &-1.0);
                            let col = lights::lighting(
                                shape.clone().material,
                                shape.clone(),
                                light,
                                pnt,
                                eye,
                                nrm,
                                false,
                            );
                            c = canvas::pixel_write(c, x, y, col);
                        }
                    }
                }
            }
        }
    }

    let duration1 = start1.elapsed();
    println!("Time to calculate data: {:?}", duration1);

    let start2 = Instant::now();
    let data = canvas::ppm_get(c);
    let duration2 = start2.elapsed();
    println!("Time to generate file: {:?}", duration2);

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
    fs::write(format!("images/sphere_lighting{}.ppm", d), string)?;
    Ok(())
}
