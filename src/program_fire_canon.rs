use chrono::prelude::*;
use std::fs;
use std::time::Instant;

use crate::canvas;
use crate::tuples;

pub fn fire_canon_main(w: u32, h: u32) {
    println!("fire canon");
    let black = tuples::color(0.0, 0.0, 0.0);
    let mut orange = tuples::color(1.0, 1.0, 0.0);
    let mut c = canvas::pixel_canvas(w, h, black);
    // velocity is normalized to 1 unit/tick.
    let mut proj = tuples::projectile(tuples::point(0.0, 1.0, 0.0), tuples::vector(5.0, 10.0, 0.0));

    //gravity -0.1 unit/tick, and wind is -0.01 unit/tick.
    let env = tuples::environment(
        tuples::vector(0.0, -0.1, 0.0),
        tuples::vector(-0.01, 0.0, 0.0),
    );

    let mut tick_count = 1;
    println!("Projectile Velocity {:?}", proj.velocity);
    println!("Environment Gravity {:?}", env.gravity);
    println!("Environment Wind {:?}", env.wind);
    println!(" {}", c.length);

    let start1 = Instant::now();
    while c.height as f64 > proj.position.y {
        let y = (c.height as f64 - proj.position.y) as u32;
        //println!(
        //    "Tick: {:?}. Projectile Position ({:?},{:?}). Velocity:{:?}",
        //    tick_count, proj.position.x, proj.position.y, proj.velocity
        //);
        c = canvas::pixel_write(c, &(proj.position.x as u32), &y, orange);
        tick_count = tick_count + 1;
        if orange.red > 0.01 {
            orange.red = orange.red - 0.01;
        }
        if orange.green > 0.01 {
            orange.green = orange.green - 0.01;
        }
        proj = env.tick(&proj);
    }

    let duration1 = start1.elapsed();
    println!(
        "Ticks: {:?}. Time to calculate data: {:?}",
        tick_count, duration1
    );

    let start2 = Instant::now();
    let data = canvas::ppm_get(&c);
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
    fs::write(format!("images/firecanon{}.ppm", d), string)?;
    Ok(())
}

fn test_color(mut c: canvas::PixelCanvas, i: u32, col: tuples::Color) -> String {
    c = canvas::pixel_write(c, &0, &0, col);
    canvas::str_from_color_get(c.data[i as usize])
}
