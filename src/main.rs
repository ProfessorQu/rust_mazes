#![allow(unused)]
#![windows_subsystem = "windows"]

mod helpers;
mod maze;

use std::{time::Duration, thread};

use maze::Maze;
use rand::{thread_rng, Rng};

pub const NODE_SIZE: usize = 10;
pub const NODE_SIZE_I: i32 = NODE_SIZE as i32;
pub const GRID_WIDTH: usize = 192;
pub const GRID_HEIGHT: usize = 102;

fn main() {
    let mut maze = Maze::new();

    let start_x = thread_rng().gen_range(0..GRID_WIDTH);
    let start_y = thread_rng().gen_range(0..GRID_HEIGHT);

    maze.init(start_x, start_y);

    let (mut rl, thread) = raylib::init()
        .size(
            (GRID_WIDTH * NODE_SIZE) as i32,
            (GRID_HEIGHT * NODE_SIZE) as i32,
        )
        .title("Maze")
        .build();

    while !rl.window_should_close() {
        if maze.complete() {
            thread::sleep(Duration::from_secs(2));

            let start_x = thread_rng().gen_range(0..GRID_WIDTH);
            let start_y = thread_rng().gen_range(0..GRID_HEIGHT);

            maze.reset();
            maze.init(start_x, start_y);
        }

        let mut d = rl.begin_drawing(&thread);

        for _ in 0..10 {
            maze.generate();
        }
        
        maze.draw(&mut d);

    }
}
