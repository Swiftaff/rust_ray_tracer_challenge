use read_input::prelude::*;

mod camera;
mod canvas;
mod intersections;
mod lights;
mod materials;
mod matrices;
mod planes;
mod program_chapter_9_planes;
mod program_fire_canon;
mod program_sphere_lighting;
mod program_sphere_outline;
mod program_world;
mod rays;
mod shapes;
mod spheres;
mod transformations;
mod tuples;
mod worlds;

const default_size_indexes: [usize; 5] = [3, 1, 2, 1, 2];

fn main() {
    let sizes_arr: [[u32; 2]; 4] = [[100, 50], [200, 100], [400, 200], [500, 250]];
    let mut x = sizes_arr[0][0];
    let mut y = sizes_arr[0][1];
    let mut index: usize = 0;

    let mut size: u32 = getSelectedSize();
    let mut program: u32 = getSelectedProgram();
    index = getSizeForProgram(size, program);
    x = sizes_arr[index][0];
    y = sizes_arr[index][1];
    runSelectedProgram(program, x, y);
}

fn getSelectedSize() -> u32 {
    let message: String = String::from(
        "Choose image size to render (square shapes use just x value)
0. use defaults
1. Tiny(100,50)
2. Small(200,100)
3. Medium(400,200)
4. Large(500,250)
? ",
    );
    input::<u32>().msg(message).get()
}

fn getSelectedProgram() -> u32 {
    let message: String = String::from(
        "Choose a program
0. program_fire_canon(500,500)
1. program_sphere_outline(200,200)
2. program_sphere_lighting(400,400)
3. world_main(200,100)
4. program_chapter_9_planes(400,200)
? ",
    );
    input::<u32>().msg(message).get()
}

fn getSizeForProgram(mut size: u32, progNum: u32) -> usize {
    let mut index: usize = 0;
    if size == 0 {
        index = default_size_indexes[progNum as usize];
    } else {
        index = (size - 1) as usize
    }
    index
}

fn runSelectedProgram(program: u32, x: u32, y: u32) {
    if program == 0 {
        program_fire_canon::fire_canon_main(x, y);
    } else if program == 1 {
        program_sphere_outline::sphere_outline_main(x, x);
    } else if program == 2 {
        program_sphere_lighting::sphere_lighting_main(x, x);
    } else if program == 3 {
        program_world::world_main(x, y);
    } else if program == 4 {
        program_chapter_9_planes::world_main(x, y);
    }
}
