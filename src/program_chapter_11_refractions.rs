extern crate image;

use chrono::prelude::*;
use last_git_commit::LastGitCommit;
use std::f64::consts::PI;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;

use crate::camera;
use crate::lights;
use crate::materials;
use crate::patterns;
use crate::planes;
use crate::shapes;
use crate::spheres;
use crate::transformations;
use crate::tuples;
use crate::worlds;

const PROGRAM_NAME: &str = "chapter_11b";

pub fn world_main(index: usize, program: u32, w: u32, h: u32) {
    println!("{} patterns", PROGRAM_NAME.to_string());
    let start1 = Instant::now();

    let mut world = worlds::world_default();
    world.objects = vec![
        shape_floor(),
        //shape_wall_behind(),
        //shape_wall_behind_right(),
        shape_sphere_middle(),
        shape_sphere_right(),
        shape_sphere_right2(),
        shape_sphere_left(),
        shape_sphere_left2(),
        shape_sphere_left3(),
    ];

    world.light = vec![lights::LightPoint {
        position: tuples::point(-3.0, 2.0, 0.5),
        intensity: tuples::COLOR_WHITE,
    }];

    let mut c = camera::camera(w, h, PI / 3.0);
    let from = tuples::point(-3.0, 2.0, -5.0);
    let to = tuples::point(0.0, 1.0, 0.0);
    let up = tuples::vector(0.0, 1.0, 0.0);
    c.transform = transformations::view_transform(&from, &to, &up);
    let image = c.render_percent_message(world, 0.01);
    let duration1 = start1.elapsed();
    println!("Time to calculate data: {:?}", duration1);

    let start2 = Instant::now();
    //let data_ppm = canvas::ppm_get(image.clone());
    let data_png = image.png_get();
    let duration2 = start2.elapsed();
    println!("Time to generate file data: {:?}", duration2);

    //logging
    let lgc = LastGitCommit::new().build().unwrap();
    let message = lgc.message().unwrap();
    let utc = Utc::now();
    let d = utc.format("%Y-%m-%d-%H-%M").to_string();
    let log_txt = format!(
        "{:?} {:?} Size: {:?} Program: {:?} Time to generate: {:?} Time to save: {:?}",
        message, d, index, program, duration1, duration2
    );
    save_log(log_txt);

    let start3 = Instant::now();

    //let f = save_ppm(d, data_ppm);
    //let _f = match f {
    //    Ok(file) => file,
    //    Err(error) => panic!("Problem saving the ppm file: {:?}", error),
    //};

    let f2 = save_png(d, data_png);
    let _f2 = match f2 {
        Ok(file) => file,
        Err(error) => panic!("Problem saving the png file: {:?}", error),
    };
    let duration3 = start3.elapsed();
    println!("Time to save file: {:?}", duration3);
}

fn save_ppm(d: String, string: String) -> std::io::Result<()> {
    fs::write(
        format!("images/{}_{}.ppm", PROGRAM_NAME.to_string(), d),
        string,
    )?;
    Ok(())
}

fn save_png(d: String, imgbuf: image::RgbImage) -> std::io::Result<()> {
    imgbuf
        .save(format!("images/{}_{}.png", PROGRAM_NAME.to_string(), d))
        .unwrap();
    Ok(())
}

fn save_log(txt: String) {
    let mut file = OpenOptions::new()
        .append(true)
        .open("benches/tests.txt")
        .unwrap();
    if let Err(e) = writeln!(file, "{}", txt) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

pub fn material_floor() -> materials::Material {
    let mut mat = materials::MATERIAL_DEFAULT;
    mat.pattern = Some(patterns::PATTERN_DEFAULT);
    mat.color = tuples::color(1.0, 0.9, 0.9);
    mat.specular = 0.0;
    mat
}

pub fn shape_floor() -> shapes::Shape {
    let mut shape = planes::plane();
    shape.material = material_floor();
    let c1 = tuples::color(0.8, 0.8, 0.8);
    let c2 = tuples::color(0.5, 0.5, 0.5);
    let mut pat = patterns::checkers_pattern(c1, c2);
    let mut mat = materials::MATERIAL_DEFAULT;
    pat.transform = transformations::matrix4_transform_chain(
        &(vec![
            transformations::matrix4_scaling(2.0, 2.0, 2.0),
            transformations::matrix4_translation(0.5, 0.0, 0.0),
        ]),
    );
    mat.pattern = Some(pat);
    mat.reflective = 0.5;
    shape.material = mat;
    shape
}

pub fn shape_wall_behind() -> shapes::Shape {
    let mut shape = planes::plane();
    let t1 = transformations::matrix4_rotation_x_rad(PI / -2.0);
    let t2 = transformations::matrix4_translation(0.0, 0.0, 2.0);
    shape.transform = transformations::matrix4_transform_chain(&(vec![t1, t2]));
    let mut mat = material_floor();
    let mut pat = patterns::PATTERN_PINK;
    pat.transform = transformations::matrix4_rotation_y_rad(PI / 4.0);
    mat.pattern = Some(pat);
    shape.material = mat;
    shape
}

pub fn shape_wall_behind_right() -> shapes::Shape {
    let mut shape = planes::plane();
    let t1 = transformations::matrix4_rotation_x_rad(PI / -2.0);
    let t2 = transformations::matrix4_rotation_y_rad(PI / 2.0);
    let t3 = transformations::matrix4_translation(2.5, 0.0, 2.0);
    shape.transform = transformations::matrix4_transform_chain(&(vec![t1, t2, t3]));
    let mut mat = material_floor();
    let mut pat = patterns::checkers_pattern(tuples::COLOR_WHITE, tuples::COLOR_BLACK);
    pat.transform = transformations::matrix4_transform_chain(
        &(vec![
            transformations::matrix4_scaling(0.2, 5.0, 0.2),
            transformations::matrix4_rotation_y_rad(PI / 16.0),
        ]),
    );
    mat.pattern = Some(pat);
    shape.material = mat;
    shape
}

pub fn shape_sphere_middle() -> shapes::Shape {
    let mut shape = spheres::sphere_glass();
    shape.transform = transformations::matrix4_translation(-0.5, 1.0, 0.5);
    shape.material = materials::MATERIAL_DEFAULT;
    shape.material.transparency = 1.0;
    shape.material.reflective = 0.9;
    shape.material.refractive_index = 2.0;
    shape.material.diffuse = 0.0;
    shape.material.shininess = 300.0;
    shape.material.specular = 0.9;
    //shape.material.color = tuples::color(0.1, 0.3, 0.8);
    //let mut mat = materials::MATERIAL_DEFAULT;
    //mat.color = tuples::color(0.1, 1.0, 0.5);
    //mat.diffuse = 0.8;
    //mat.specular = 0.7;
    //let mut pat = patterns::PATTERN_DEFAULT;
    //pat.a = tuples::color(0.0, 0.8, 0.0);
    //pat.b = tuples::color(0.0, 0.9, 0.5);
    //pat.transform = transformations::matrix4_transform_chain(vec![
    //    transformations::matrix4_scaling(0.1, 1.0, 0.1),
    //    transformations::matrix4_translation(0.5, 0.0, 0.0),
    //    transformations::matrix4_rotation_y_rad(PI / 4.0),
    //    transformations::matrix4_rotation_x_rad(PI / 4.0),
    //]);
    //mat.pattern = Some(pat);
    //mat.reflective = 0.8;
    //shape.material = mat;
    shape
}

pub fn shape_sphere_right() -> shapes::Shape {
    let mut shape = spheres::sphere();
    let t1 = transformations::matrix4_translation(0.0, 0.75, 8.0);
    let t2 = transformations::matrix4_scaling(0.75, 0.75, 0.75);
    shape.transform = transformations::matrix4_transform_chain(&vec![t2, t1]);

    let mut mat = materials::MATERIAL_DEFAULT;
    mat.color = tuples::color(1.0, 0.1, 0.1);
    mat.diffuse = 0.7;
    mat.specular = 0.3;
    shape.material = mat;
    shape
}

pub fn shape_sphere_right2() -> shapes::Shape {
    let mut shape = spheres::sphere();
    let t1 = transformations::matrix4_translation(1.0, 0.5, 8.0);
    let t2 = transformations::matrix4_scaling(1.0, 1.0, 1.0);
    shape.transform = transformations::matrix4_transform_chain(&vec![t2, t1]);

    let mut mat = materials::MATERIAL_DEFAULT;
    mat.color = tuples::color(0.1, 0.6, 0.8);
    mat.diffuse = 0.7;
    mat.specular = 0.3;
    let mut pat = patterns::PATTERN_PINK;
    pat.a = tuples::color(0.0, 0.0, 0.8);
    pat.b = tuples::color(0.1, 0.1, 0.4);
    pat.transform = transformations::matrix4_scaling(0.25, 0.25, 0.25);
    mat.pattern = Some(pat);
    shape.material = mat;
    shape
}

pub fn shape_sphere_left() -> shapes::Shape {
    let mut shape = spheres::sphere();
    let t1 = transformations::matrix4_translation(-1.0, 0.5, 3.0);
    let t2 = transformations::matrix4_scaling(0.5, 0.5, 0.5);
    shape.transform = transformations::matrix4_transform_chain(&vec![t2, t1]);

    let mut mat = materials::MATERIAL_DEFAULT;
    mat.color = tuples::color(1.0, 0.8, 0.1);
    mat.diffuse = 0.7;
    mat.specular = 0.3;
    shape.material = mat;
    shape
}

pub fn shape_sphere_left2() -> shapes::Shape {
    let mut shape = spheres::sphere();
    let t1 = transformations::matrix4_translation(0.0, 0.5, 3.0);
    let t2 = transformations::matrix4_scaling(0.5, 0.5, 0.5);
    shape.transform = transformations::matrix4_transform_chain(&vec![t2, t1]);

    let mut mat = materials::MATERIAL_DEFAULT;
    mat.color = tuples::color(1.0, 0.8, 0.1);
    mat.diffuse = 0.7;
    mat.specular = 0.3;
    shape.material = mat;
    shape
}

pub fn shape_sphere_left3() -> shapes::Shape {
    let mut shape = spheres::sphere();
    let t1 = transformations::matrix4_translation(1.0, 0.5, 3.0);
    let t2 = transformations::matrix4_scaling(0.5, 0.5, 0.5);
    shape.transform = transformations::matrix4_transform_chain(&vec![t2, t1]);

    let mut mat = materials::MATERIAL_DEFAULT;
    mat.color = tuples::color(1.0, 0.8, 0.1);
    mat.diffuse = 0.7;
    mat.specular = 0.3;
    shape.material = mat;
    shape
}
