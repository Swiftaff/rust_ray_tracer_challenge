use std::time::{Duration, Instant};

mod tuples;

fn main(){
    println!("fire canon");

    //let c = pixelCanvas(500, 500);
    let mut orange = tuples::color(1.0,1.0,0.0);

    // velocity is normalized to 1 unit/tick.
    let mut proj = tuples::projectile(tuples::point(0.0, 1.0, 0.0), tuples::vector(5.0, 10.0, 0.0));

    //gravity -0.1 unit/tick, and wind is -0.01 unit/tick.
    let env = tuples::environment(tuples::vector(0.0, -0.1, 0.0), tuples::vector(-0.01, 0.0, 0.0));

    let mut tick_count = 1;
    println!("Projectile Velocity {:?}", proj.velocity);
    println!("Environment Gravity {:?}", env.gravity);
    println!("Environment Wind {:?}", env.wind);
    println!(" ");

    let start = Instant::now();
    while proj.position.y > 0.0 {
        //println!("Tick: {:?}. Projectile Position ({:?},{:?}). Velocity:{:?}", tick_count, proj.position.x, proj.position.y, proj.velocity);
        //let newC = pixel_write(c, Math.floor(proj.position.x), Math.floor(c.height - proj.position.y), orange);
        //c = newC;
        tick_count = tick_count + 1;
        orange.red = orange.red - 0.01;
        orange.green = orange.green - 0.01;
        proj = tuples::tick(&env, &proj);
    }

    let duration = start.elapsed();
    println!("Ticks: {:?}. Time elapsed is: {:?}", tick_count, duration);
}