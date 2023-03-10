#![allow(unused)]
#![windows_subsystem = "windows"]

mod helpers;
mod maze;
mod mazes;

use std::time::Instant;

use maze::Maze;
use mazes::*;
use rand::{distributions::Standard, prelude::Distribution};

pub const NODE_SIZE: usize = 10;
pub const NODE_SIZE_I: i32 = NODE_SIZE as i32;
pub const GRID_WIDTH: usize = 192;
pub const GRID_HEIGHT: usize = 102;

#[derive(Debug)]
pub enum Algorithm {
    DepthFirstSearch,
    BinaryTree,
    HuntAndKill,
    Prim,
    Kruskal,
    AldousBroder,
}

impl Algorithm {
    fn next(&self) -> Self {
        match self {
            Algorithm::DepthFirstSearch => Algorithm::BinaryTree,
            Algorithm::BinaryTree => Algorithm::HuntAndKill,
            Algorithm::HuntAndKill => Algorithm::Prim,
            Algorithm::Prim => Algorithm::Kruskal,
            Algorithm::Kruskal => Algorithm::AldousBroder,
            Algorithm::AldousBroder => Algorithm::DepthFirstSearch,
        }
    }
}

impl Distribution<Algorithm> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Algorithm {
        match rng.gen_range(0..6) {
            0 => Algorithm::DepthFirstSearch,
            1 => Algorithm::BinaryTree,
            2 => Algorithm::HuntAndKill,
            3 => Algorithm::Prim,
            4 => Algorithm::Kruskal,
            _ => Algorithm::AldousBroder,
        }
    }
}

fn main() {
    let mut depth = DepthFirstSearch::new();
    let mut binary = BinaryTree::new();
    let mut hunt = HuntAndKill::new();
    let mut prim = Prim::new();
    let mut kruskal = Kruskal::new();
    let mut aldous = AldousBroder::new();

    depth.reset();
    binary.reset();
    hunt.reset();
    prim.reset();
    kruskal.reset();
    aldous.reset();

    let mut current = rand::random();

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
                depth.update(&mut now, &mut current, &mut rl, &thread, 110)
            }
            Algorithm::BinaryTree => binary.update(&mut now, &mut current, &mut rl, &thread, 25),
            Algorithm::HuntAndKill => hunt.update(&mut now, &mut current, &mut rl, &thread, 50),
            Algorithm::Prim => prim.update(&mut now, &mut current, &mut rl, &thread, 30),
            Algorithm::Kruskal => kruskal.update(&mut now, &mut current, &mut rl, &thread, 120),
            Algorithm::AldousBroder => {
                aldous.update(&mut now, &mut current, &mut rl, &thread, 2000)
            }
        }
    }
}
