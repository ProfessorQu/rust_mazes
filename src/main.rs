#![allow(unused)]

use rand::{seq::SliceRandom, thread_rng};
use raylib::prelude::*;

pub const NODE_SIZE: usize = 10;
pub const NODE_SIZE_I: i32 = NODE_SIZE as i32;
pub const GRID_WIDTH: usize = 50;
pub const GRID_HEIGHT: usize = 50;

enum Direction {
    Up(Pos),
    Down(Pos),
    Left(Pos),
    Right(Pos),
}

#[derive(Clone, Copy, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

struct Node {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Node {
    fn new(up: bool, down: bool, left: bool, right: bool) -> Self {
        Self {
            up,
            down,
            left,
            right,
        }
    }
}

fn get_neigbors(pos: Pos) -> Vec<Direction> {
    let mut neigbors = vec![];
    if pos.x > 0 {
        neigbors.push(Direction::Left(Pos::new(pos.x - 1, pos.y)));
    }
    if pos.x < GRID_WIDTH - 1 {
        neigbors.push(Direction::Right(Pos::new(pos.x + 1, pos.y)));
    }
    if pos.y > 0 {
        neigbors.push(Direction::Up(Pos::new(pos.x, pos.y - 1)));
    }
    if pos.y < GRID_HEIGHT - 1 {
        neigbors.push(Direction::Down(Pos::new(pos.x, pos.y + 1)));
    }

    neigbors.shuffle(&mut thread_rng());

    neigbors
}

fn generate_maze(pos: Pos, nodes: &mut Vec<Vec<Node>>, visited: &mut Vec<Pos>) {
    for neigbor in get_neigbors(pos) {
        match neigbor {
            Direction::Left(next_pos) => {
                if !visited.contains(&next_pos) {
                    nodes[pos.x][pos.y].left = false;
                    nodes[next_pos.x][next_pos.y].right = false;
                    visited.push(next_pos);

                    generate_maze(next_pos, nodes, visited);
                }
            }
            Direction::Right(next_pos) => {
                if !visited.contains(&next_pos) {
                    nodes[pos.x][pos.y].right = false;
                    nodes[next_pos.x][next_pos.y].left = false;
                    visited.push(next_pos);

                    generate_maze(next_pos, nodes, visited);
                }
            }
            Direction::Up(next_pos) => {
                if !visited.contains(&next_pos) {
                    nodes[pos.x][pos.y].up = false;
                    nodes[next_pos.x][next_pos.y].down = false;
                    visited.push(next_pos);

                    generate_maze(next_pos, nodes, visited);
                }
            }
            Direction::Down(next_pos) => {
                if !visited.contains(&next_pos) {
                    nodes[pos.x][pos.y].down = false;
                    nodes[next_pos.x][next_pos.y].up = false;
                    visited.push(next_pos);

                    generate_maze(next_pos, nodes, visited);
                }
            }
        }
    }
}

fn draw(d: &mut RaylibDrawHandle, nodes: &[Vec<Node>]) {
    d.clear_background(Color::WHITE);

    for (x, row) in nodes.iter().enumerate() {
        for y in 0..row.len() {
            let node = &nodes[x][y];
            let x = x as i32 * NODE_SIZE_I;
            let y = y as i32 * NODE_SIZE_I;
            if node.up {
                d.draw_line(x, y, x + NODE_SIZE_I, y, Color::BLACK);
            }
            if node.left {
                d.draw_line(x, y, x, y + NODE_SIZE_I, Color::BLACK);
            }
        }
    }
}

fn main() {
    let mut nodes = vec![];
    let mut visited = vec![];

    for x in 0..GRID_WIDTH {
        nodes.push(vec![]);
        for _y in 0..GRID_HEIGHT {
            nodes[x].push(Node::new(true, true, true, true));
        }
    }

    generate_maze(Pos::new(0, 0), &mut nodes, &mut visited);

    let (mut rl, thread) = raylib::init()
        .size(
            (GRID_WIDTH * NODE_SIZE) as i32,
            (GRID_HEIGHT * NODE_SIZE) as i32,
        )
        .title("Maze")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        draw(&mut d, &nodes);
    }
}
