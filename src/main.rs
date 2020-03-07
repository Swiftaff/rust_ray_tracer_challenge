mod tuples;

fn main(){
    println!("fireCanon");

    //let c = pixelCanvas(500, 500);
    let orange = tuples::color(1.0,1.0,0.0);

    // velocity is normalized to 1 unit/tick.
    let proj = tuples::projectile(tuples::point(0.0, 1.0, 0.0), tuples::vector(5.0, 10.0, 0.0));

    //gravity -0.1 unit/tick, and wind is -0.01 unit/tick.
    let env = tuples::environment(tuples::vector(0.0, -0.1, 0.0), tuples::vector(-0.01, 0.0, 0.0));

    let tickCount = 1;
    println!("Projectile Velocity {:?}", proj.velocity);
    println!("Environment Gravity {:?}", env.gravity);
    println!("Environment Wind {:?}", env.wind);
    println!(" ");
}