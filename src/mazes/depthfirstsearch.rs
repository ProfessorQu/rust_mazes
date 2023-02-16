use std::collections::HashSet;

use rand::{thread_rng, Rng};
use raylib::prelude::*;

use crate::{helpers::*, maze::Maze, GRID_HEIGHT, GRID_WIDTH, NODE_SIZE_I};

#[derive(Clone)]
pub struct DepthFirstSearch {
    nodes: Vec<Vec<Node>>,
    visited: HashSet<Pos>,
    stack: Vec<Pos>,
}

impl DepthFirstSearch {
    fn handle_neighbor(&mut self, pos: Pos, neighbor: Direction) {
        let next_pos = neighbor.get_pos();

        self.visited.insert(next_pos);

        self.stack.push(pos);
        self.stack.push(next_pos);
    }
}

impl Maze for DepthFirstSearch {
    fn new() -> Self {
        Self {
            nodes: vec![],
            visited: HashSet::new(),
            stack: vec![],
        }
    }

    fn complete(&self) -> bool {
        self.stack.is_empty()
    }

    fn reset(&mut self) {
        self.nodes.clear();
        for x in 0..GRID_WIDTH {
            self.nodes.push(vec![]);
            for _y in 0..GRID_HEIGHT {
                self.nodes[x].push(Node::new());
            }
        }

        self.stack.clear();
        self.visited.clear();

        let start_x = thread_rng().gen_range(0..GRID_WIDTH);
        let start_y = thread_rng().gen_range(0..GRID_HEIGHT);
        let start_pos = Pos::new(start_x, start_y);

        self.stack.push(start_pos);
        self.visited.insert(start_pos);
    }

    fn generate(&mut self) {
        if let Some(pos) = self.stack.pop() {
            let neighbor = pos.get_random_neighbor(&self.visited);
            if let Some(neighbor) = neighbor {
                pos.make_connection(&neighbor, &mut self.nodes);
                self.handle_neighbor(pos, neighbor);
            }
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::BLACK);

        for (x, row) in self.nodes.iter().enumerate() {
            for y in 0..row.len() {
                let node = &self.nodes[x][y];
                let pos = Pos::new(x, y);

                let screen_x = x as i32 * NODE_SIZE_I;
                let screen_y = y as i32 * NODE_SIZE_I;

                if self.visited.contains(&pos) {
                    if self.stack.contains(&pos) {
                        d.draw_rectangle(
                            screen_x,
                            screen_y,
                            NODE_SIZE_I,
                            NODE_SIZE_I,
                            Color::GREEN,
                        );
                    } else {
                        d.draw_rectangle(
                            screen_x,
                            screen_y,
                            NODE_SIZE_I,
                            NODE_SIZE_I,
                            Color::WHITE,
                        );
                    }
                    if node.up {
                        d.draw_line(
                            screen_x,
                            screen_y,
                            screen_x + NODE_SIZE_I,
                            screen_y,
                            Color::BLACK,
                        );
                    }
                    if node.left {
                        d.draw_line(
                            screen_x,
                            screen_y,
                            screen_x,
                            screen_y + NODE_SIZE_I,
                            Color::BLACK,
                        );
                    }
                }
            }
        }
    }
}
