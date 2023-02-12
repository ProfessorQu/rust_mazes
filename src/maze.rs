use rand::{seq::SliceRandom, thread_rng};
use raylib::prelude::*;

use crate::{NODE_SIZE_I, GRID_WIDTH, GRID_HEIGHT};

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

    fn get_neigbors(&self, visited: &[Pos]) -> Vec<Direction> {
        let mut neigbors = vec![];
        if self.x > 0 {
            let new_pos = Pos::new(self.x - 1, self.y);
            if !visited.contains(&new_pos) {
                neigbors.push(Direction::Left(new_pos));
            }
        }
        if self.x < GRID_WIDTH - 1 {
            let new_pos = Pos::new(self.x + 1, self.y);
            if !visited.contains(&new_pos) {
                neigbors.push(Direction::Right(new_pos));
            }
        }
        if self.y > 0 {
            let new_pos = Pos::new(self.x, self.y - 1);
            if !visited.contains(&new_pos) {
                neigbors.push(Direction::Up(new_pos));
            }
        }
        if self.y < GRID_HEIGHT - 1 {
            let new_pos = Pos::new(self.x, self.y + 1);
            if !visited.contains(&new_pos) {
                neigbors.push(Direction::Down(new_pos));
            }
        }

        neigbors.shuffle(&mut thread_rng());

        neigbors
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
}

impl Maze {
    pub fn new() -> Self {
        let mut nodes = vec![];
        let mut visited = vec![];
        for x in 0..GRID_WIDTH {
            nodes.push(vec![]);
            for _y in 0..GRID_HEIGHT {
                nodes[x].push(Node::new(true, true, true, true));
            }
        }
        Self { nodes, visited }
    }

    pub fn generate(&mut self, start_pos: Pos) {
        let mut stack = vec![start_pos];
        self.visited.push(start_pos);

        while let Some(pos) = stack.pop() {
            let neigbors = pos.get_neigbors(&self.visited);

            if !neigbors.is_empty() {
                match neigbors[0] {
                    Direction::Left(next_pos) => {
                        self.nodes[pos.x][pos.y].left = false;
                        self.nodes[next_pos.x][next_pos.y].right = false;
                        self.visited.push(next_pos);

                        stack.push(pos);
                        stack.push(next_pos);
                    }
                    Direction::Right(next_pos) => {
                        self.nodes[pos.x][pos.y].right = false;
                        self.nodes[next_pos.x][next_pos.y].left = false;
                        self.visited.push(next_pos);

                        stack.push(pos);
                        stack.push(next_pos);
                    }
                    Direction::Up(next_pos) => {
                        self.nodes[pos.x][pos.y].up = false;
                        self.nodes[next_pos.x][next_pos.y].down = false;
                        self.visited.push(next_pos);

                        stack.push(pos);
                        stack.push(next_pos);
                    }
                    Direction::Down(next_pos) => {
                        self.nodes[pos.x][pos.y].down = false;
                        self.nodes[next_pos.x][next_pos.y].up = false;
                        self.visited.push(next_pos);

                        stack.push(pos);
                        stack.push(next_pos);
                    }
                }
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::WHITE);

        for (x, row) in self.nodes.iter().enumerate() {
            for y in 0..row.len() {
                let node = &self.nodes[x][y];
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
}
