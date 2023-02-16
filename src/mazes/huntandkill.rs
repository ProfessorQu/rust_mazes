use std::collections::HashSet;

use raylib::prelude::*;

use crate::{helpers::*, maze::Maze, GRID_HEIGHT, GRID_WIDTH, NODE_SIZE_I};

#[derive(Clone)]
pub struct HuntAndKill {
    nodes: Vec<Vec<Node>>,
    visited: HashSet<Pos>,
    hunting_pos: Pos,
    killing_pos: Pos,
    hunting: bool,
}

impl HuntAndKill {
    fn handle_neighbor(&mut self, neighbor: Direction) {
        let next_pos = neighbor.get_pos();

        self.visited.insert(next_pos);

        self.killing_pos = next_pos;
    }
}

impl Maze for HuntAndKill {
    fn new() -> Self {
        Self {
            nodes: vec![],
            visited: HashSet::new(),
            hunting_pos: Pos::new(0, 0),
            killing_pos: Pos::new(0, 0),
            hunting: true,
        }
    }

    fn complete(&self) -> bool {
        self.hunting_pos.y == GRID_HEIGHT
    }

    fn reset(&mut self) {
        self.nodes.clear();
        for x in 0..GRID_WIDTH {
            self.nodes.push(vec![]);
            for _y in 0..GRID_HEIGHT {
                self.nodes[x].push(Node::new());
            }
        }

        let pos = Pos::new(0, 0);

        self.visited.clear();
        self.visited.insert(pos);

        self.hunting_pos = pos;
        self.killing_pos = pos;

        self.hunting = true;
    }

    fn generate(&mut self) {
        if !self.complete() {
            if self.hunting {
                let neighbor = self.hunting_pos.get_random_neighbor(&self.visited);
                if neighbor.is_some() {
                    self.killing_pos = self.hunting_pos;
                    self.hunting = false;
                } else {
                    self.hunting_pos.x += 1;
                    if self.hunting_pos.x >= GRID_WIDTH {
                        self.hunting_pos.y += 1;
                        self.hunting_pos.x = 0;
                    }
                }
            } else {
                let neighbor = self.killing_pos.get_random_neighbor(&self.visited);
                if let Some(neighbor) = neighbor {
                    self.killing_pos.make_connection(&neighbor, &mut self.nodes);
                    self.handle_neighbor(neighbor);
                } else {
                    self.hunting = true;
                }
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
                    if pos.y > self.hunting_pos.y
                        || (pos.y == self.hunting_pos.y && pos.x > self.hunting_pos.x)
                    {
                        d.draw_rectangle(screen_x, screen_y, NODE_SIZE_I, NODE_SIZE_I, Color::GOLD);
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
