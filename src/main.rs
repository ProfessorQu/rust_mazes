#![allow(unused)]
// #![windows_subsystem = "windows"]

mod helpers;
mod maze;
mod mazes;

use std::{thread, time::{Duration, Instant}};

use maze::Maze;
use mazes::{BinaryTree, DepthFirstSearch};

pub const NODE_SIZE: usize = 10;
pub const NODE_SIZE_I: i32 = NODE_SIZE as i32;
pub const GRID_WIDTH: usize = 192;
pub const GRID_HEIGHT: usize = 102;

enum Algorithm {
    DepthFirstSearch,
    BinaryTree,
}

fn main() {
    let mut depth = DepthFirstSearch::new();
    let mut binary = BinaryTree::new();

    depth.reset();
    binary.reset();

    let mut current = Algorithm::DepthFirstSearch;

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
                if depth.complete() {
                    println!("Depth first search took: {:?}", now.elapsed());

                    thread::sleep(Duration::from_secs(2));
                    depth.reset();

                    current = Algorithm::BinaryTree;
                    now = Instant::now();
                }

                let mut d = rl.begin_drawing(&thread);

                for _ in 0..50 {
                    depth.generate();
                }

                depth.draw(&mut d);
            }
            Algorithm::BinaryTree => {
                if binary.complete() {
                    println!("Binary tree took: {:?}", now.elapsed());

                    thread::sleep(Duration::from_secs(2));
                    binary.reset();

                    current = Algorithm::DepthFirstSearch;
                    now = Instant::now();
                }

                let mut d = rl.begin_drawing(&thread);

                for _ in 0..30 {
                    binary.generate();
                }

                binary.draw(&mut d);
            }
        }
    }
}
