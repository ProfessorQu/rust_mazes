use std::collections::HashSet;

use raylib::prelude::*;

use crate::{helpers::*, GRID_HEIGHT, GRID_WIDTH, NODE_SIZE_I};

#[derive(Clone)]
pub struct Maze {
    nodes: Vec<Vec<Node>>,
    visited: HashSet<Pos>,
    stack: Vec<Pos>,
}

impl Maze {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            visited: HashSet::new(),
            stack: vec![],
        }
    }

    pub fn complete(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn reset(&mut self, start_pos: Pos) {
        self.nodes = vec![];
        for x in 0..GRID_WIDTH {
            self.nodes.push(vec![]);
            for _y in 0..GRID_HEIGHT {
                self.nodes[x].push(Node::new());
            }
        }

        self.stack = vec![];
        self.visited = HashSet::new();

        self.stack.push(start_pos);
        self.visited.insert(start_pos);
    }

    fn handle_neighbor(&mut self, pos: Pos, neighbor: Direction) {
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

        self.visited.insert(next_pos);

        self.stack.push(pos);
        self.stack.push(next_pos);
    }

    pub fn generate(&mut self) {
        if let Some(pos) = self.stack.pop() {
            let neighbor = pos.get_random_neighbor(&self.visited);
            if let Some(neighbor) = neighbor {
                self.handle_neighbor(pos, neighbor);
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
                    if self.stack.contains(&pos) {
                        d.draw_rectangle(x, y, NODE_SIZE_I, NODE_SIZE_I, Color::GREEN);
                    } else {
                        d.draw_rectangle(x, y, NODE_SIZE_I, NODE_SIZE_I, Color::WHITE);
                    }
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
}
