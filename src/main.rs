use std::time::Instant;

mod tuples;
mod canvas;

fn main(){
    println!("fire canon");
    let black = tuples::color(0.0, 0.0, 0.0);
    let mut orange = tuples::color(1.0,1.0,0.0);
    let mut c = canvas::pixel_canvas(500, 500, black);
    // velocity is normalized to 1 unit/tick.
    let mut proj = tuples::projectile(tuples::point(0.0, 1.0, 0.0), tuples::vector(5.0, 10.0, 0.0));

    //gravity -0.1 unit/tick, and wind is -0.01 unit/tick.
    let env = tuples::environment(tuples::vector(0.0, -0.1, 0.0), tuples::vector(-0.01, 0.0, 0.0));

    let mut tick_count = 1;
    println!("Projectile Velocity {:?}", proj.velocity);
    println!("Environment Gravity {:?}", env.gravity);
    println!("Environment Wind {:?}", env.wind);
    println!(" {}", c.length);

    let start = Instant::now();
    while proj.position.y > 0.0 {
        let y = &c.height - proj.position.y as u32;
        //println!("Tick: {:?}. Projectile Position ({:?},{:?}). Velocity:{:?}", tick_count, proj.position.x, proj.position.y, proj.velocity);
        c = canvas::pixel_write(c, proj.position.x as u32, y, orange);
        tick_count = tick_count + 1;
        orange.red = orange.red - 0.01;
        orange.green = orange.green - 0.01;
        proj = tuples::tick(&env, &proj);
    }

    let duration = start.elapsed();
    println!("Ticks: {:?}. Time elapsed is: {:?}", tick_count, duration);
}