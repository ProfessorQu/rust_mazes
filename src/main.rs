#![allow(unused)]
#![windows_subsystem = "windows"]

mod helpers;
mod maze;
mod mazes;

use std::time::Instant;

use maze::Maze;
use mazes::*;

pub const NODE_SIZE: usize = 10;
pub const NODE_SIZE_I: i32 = NODE_SIZE as i32;
pub const GRID_WIDTH: usize = 192;
pub const GRID_HEIGHT: usize = 102;

#[derive(Debug)]
pub enum Algorithm {
    DepthFirstSearch,
    BinaryTree,
    HuntAndKill,
    RandomizedPrim,
}

impl Algorithm {
    fn next(&self) -> Self {
        match self {
            Algorithm::DepthFirstSearch => Algorithm::BinaryTree,
            Algorithm::BinaryTree => Algorithm::HuntAndKill,
            Algorithm::HuntAndKill => Algorithm::RandomizedPrim,
            Algorithm::RandomizedPrim => Algorithm::DepthFirstSearch,
        }
    }
}

fn main() {
    let mut depth = DepthFirstSearch::new();
    let mut binary = BinaryTree::new();
    let mut hunt = HuntAndKill::new();
    let mut prim = RandomizedPrim::new();

    depth.reset();
    binary.reset();
    hunt.reset();
    prim.reset();

    let mut current = Algorithm::RandomizedPrim;

    let (mut rl, thread) = raylib::init()
        .size(
            (GRID_WIDTH * NODE_SIZE) as i32,
            (GRID_HEIGHT * NODE_SIZE) as i32,
        )
        .title("Maze")
        .build();

    let mut now = Instant::now();

    while !rl.window_should_close() {
        match current {
            Algorithm::DepthFirstSearch => {
                depth.update(&mut now, &mut current, &mut rl, &thread, 110);
            }
            Algorithm::BinaryTree => {
                binary.update(&mut now, &mut current, &mut rl, &thread, 25);
            }
            Algorithm::HuntAndKill => {
                hunt.update(&mut now, &mut current, &mut rl, &thread, 50);
            }
            Algorithm::RandomizedPrim => {
                prim.update(&mut now, &mut current, &mut rl, &thread, 30);
            }
        }
    }
}
