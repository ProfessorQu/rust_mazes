#![allow(unused)]

mod maze;
use maze::{Maze, Pos};

use rand::{seq::SliceRandom, thread_rng};
use raylib::prelude::*;

pub const NODE_SIZE: usize = 10;
pub const NODE_SIZE_I: i32 = NODE_SIZE as i32;
pub const GRID_WIDTH: usize = 50;
pub const GRID_HEIGHT: usize = 50;

fn main() {
    let mut maze = Maze::new();

    maze.generate(Pos::new(0, 0));

    let (mut rl, thread) = raylib::init()
        .size(
            (GRID_WIDTH * NODE_SIZE) as i32,
            (GRID_HEIGHT * NODE_SIZE) as i32,
        )
        .title("Maze")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        maze.draw(&mut d);
    }
}
