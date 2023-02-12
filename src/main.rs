// #![allow(unused)]

mod maze;

use maze::{Maze, Pos};

pub const NODE_SIZE: usize = 80;
pub const NODE_SIZE_I: i32 = NODE_SIZE as i32;
pub const GRID_WIDTH: usize = 19;
pub const GRID_HEIGHT: usize = 10;

fn main() {
    let mut maze = Maze::new();

    maze.init(Pos::new(0, 0));

    let (mut rl, thread) = raylib::init()
        .size(
            (GRID_WIDTH * NODE_SIZE) as i32,
            (GRID_HEIGHT * NODE_SIZE) as i32,
        )
        .title("Maze")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        maze.generate();
        maze.draw(&mut d);
    }
}
