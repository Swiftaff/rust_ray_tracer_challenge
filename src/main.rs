#![allow(dead_code)]

use read_input::prelude::*;

mod camera;
mod canvas;
mod cubes;
mod intersections;
mod lights;
mod materials;
mod matrices;
mod patterns;
mod planes;
mod program_chapter_10_patterns;
mod program_chapter_11_reflection;
mod program_chapter_11_refractions;
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

const DEFAULT_SIZE_INDICES: [usize; 8] = [4, 2, 3, 2, 1, 1, 1, 1];

fn main() {
    let sizes_arr: [[u32; 2]; 11] = [
        [50, 25],
        [100, 50],
        [200, 100],
        [400, 200],
        [500, 250],
        [1000, 500],
        [2000, 1000],
        [50, 50],
        [100, 100],
        [200, 200],
        [300, 300],
    ];
    let size: u32 = get_selected_size();
    let program: u32 = get_selected_program();
    let index: usize = get_size_for_program(size, program);
    let x: u32 = sizes_arr[index][0];
    let y: u32 = sizes_arr[index][1];
    run_selected_program(index, program, x, y);
}

fn get_selected_size() -> u32 {
    let message: String = String::from(
        "Choose image size to render (square shapes use just x value)
0. use defaults
1. X-Tiny(50,25) <-benchmark a
2. Tiny(100,50)
3. Small(200,100)
4. Medium(400,200) <-benchmark b
5. Large(500,250)
6. Large(1000,500)
7. Large(2000,1000)

8. Large(50,50)
9. Large(100,100)
10. Large(200,200)
11. Large(300,300)
? ",
    );
    input::<u32>().msg(message).get()
}

fn get_selected_program() -> u32 {
    let message: String = String::from(
        "Choose a program
0. program_fire_canon(500,500)
1. program_sphere_outline(200,200)
2. program_sphere_lighting(400,400)
3. world_main(200,100)
4. program_chapter_9_planes(100,50)
5. program_chapter_10_patterns(100,50)
6. program_chapter_11_reflection(100,50)
7. program_chapter_11_refractions(100,50)
? ",
    );
    input::<u32>().msg(message).get()
}

fn get_size_for_program(size: u32, prog_num: u32) -> usize {
    let index: usize;
    if size == 0 {
        index = DEFAULT_SIZE_INDICES[prog_num as usize];
    } else {
        index = (size - 1) as usize
    }
    index
}

fn run_selected_program(index: usize, program: u32, x: u32, y: u32) {
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
    } else if program == 5 {
        program_chapter_10_patterns::world_main(x, y);
    } else if program == 6 {
        program_chapter_11_reflection::world_main(x, y);
    } else if program == 7 {
        program_chapter_11_refractions::world_main(index, program, x, y);
    }
}
