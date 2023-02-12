use rand::{seq::SliceRandom, thread_rng};
use raylib::prelude::*;

use crate::{GRID_HEIGHT, GRID_WIDTH, NODE_SIZE_I};

#[derive(Clone, Copy)]
enum Direction {
    Up(Pos),
    Down(Pos),
    Left(Pos),
    Right(Pos),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn get_neighbor(&self, visited: &[Pos]) -> Option<Direction> {
        let mut neighbors = vec![];
        if self.x > 0 {
            let new_pos = Pos::new(self.x - 1, self.y);
            if !visited.contains(&new_pos) {
                neighbors.push(Direction::Left(new_pos));
            }
        }
        if self.x < GRID_WIDTH - 1 {
            let new_pos = Pos::new(self.x + 1, self.y);
            if !visited.contains(&new_pos) {
                neighbors.push(Direction::Right(new_pos));
            }
        }
        if self.y > 0 {
            let new_pos = Pos::new(self.x, self.y - 1);
            if !visited.contains(&new_pos) {
                neighbors.push(Direction::Up(new_pos));
            }
        }
        if self.y < GRID_HEIGHT - 1 {
            let new_pos = Pos::new(self.x, self.y + 1);
            if !visited.contains(&new_pos) {
                neighbors.push(Direction::Down(new_pos));
            }
        }

        neighbors.choose(&mut thread_rng()).copied()
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

pub struct Maze {
    nodes: Vec<Vec<Node>>,
    visited: Vec<Pos>,
    stack: Vec<Pos>,
}

impl Maze {
    pub fn new() -> Self {
        let mut nodes = vec![];
        for x in 0..GRID_WIDTH {
            nodes.push(vec![]);
            for _y in 0..GRID_HEIGHT {
                nodes[x].push(Node::new(true, true, true, true));
            }
        }
        Self {
            nodes,
            visited: vec![],
            stack: vec![],
        }
    }

    pub fn init(&mut self, start_pos: Pos) {
        self.stack.push(start_pos);
        self.visited.push(start_pos);
    }

    pub fn generate(&mut self) {
        if let Some(pos) = self.stack.pop() {
            let neighbor = pos.get_neighbor(&self.visited);

            if let Some(neighbor) = neighbor {
                let next_pos = match neighbor {
                    Direction::Left(next_pos) => {
                        self.nodes[pos.x][pos.y].left = false;
                        self.nodes[next_pos.x][next_pos.y].right = false;
                        next_pos
                    }
                    Direction::Right(next_pos) => {
                        self.nodes[pos.x][pos.y].right = false;
                        self.nodes[next_pos.x][next_pos.y].left = false;
                        next_pos
                    }
                    Direction::Up(next_pos) => {
                        self.nodes[pos.x][pos.y].up = false;
                        self.nodes[next_pos.x][next_pos.y].down = false;
                        next_pos
                    }
                    Direction::Down(next_pos) => {
                        self.nodes[pos.x][pos.y].down = false;
                        self.nodes[next_pos.x][next_pos.y].up = false;
                        next_pos
                    }
                };

                self.visited.push(next_pos);

                self.stack.push(pos);
                self.stack.push(next_pos);
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::BLACK);

        for (x, row) in self.nodes.iter().enumerate() {
            for y in 0..row.len() {
                let node = &self.nodes[x][y];
                let pos = Pos::new(x, y);
                let x = x as i32 * NODE_SIZE_I;
                let y = y as i32 * NODE_SIZE_I;
                if self.visited.contains(&pos) {
                    d.draw_rectangle(x, y, NODE_SIZE_I, NODE_SIZE_I, Color::WHITE);
                    if node.up {
                        d.draw_line(x, y, x + NODE_SIZE_I, y, Color::BLACK);
                    }
                    if node.left {
                        d.draw_line(x, y, x, y + NODE_SIZE_I, Color::BLACK);
                    }
                }
            }
        }

        if let Some(pos) = self.stack.last() {
            let x = pos.x as i32 * NODE_SIZE_I;
            let y = pos.y as i32 * NODE_SIZE_I;

            d.draw_rectangle(x, y, NODE_SIZE_I, NODE_SIZE_I, Color::RED);
        }
    }
}
