use rand::random;
use raylib::prelude::*;

use crate::{helpers::*, maze::Maze, GRID_HEIGHT, GRID_WIDTH, NODE_SIZE_I};

pub struct BinaryTree {
    nodes: Vec<Vec<Node>>,
    current_pos: Pos,
}

impl Maze for BinaryTree {
    fn new() -> Self {
        Self {
            nodes: vec![],
            current_pos: Pos::new(0, 0),
        }
    }

    fn complete(&self) -> bool {
        self.current_pos.y == GRID_HEIGHT
    }

    fn reset(&mut self) {
        self.nodes = vec![];
        for x in 0..GRID_WIDTH {
            self.nodes.push(vec![]);
            for _y in 0..GRID_HEIGHT {
                self.nodes[x].push(Node::new());
            }
        }

        self.current_pos = Pos::new(0, 0);
    }

    fn generate(&mut self) {
        if !self.complete() {
            if self.current_pos.x > 0 && self.current_pos.y > 0 {
                if random() {
                    self.nodes[self.current_pos.x][self.current_pos.y].up = false;
                    self.nodes[self.current_pos.x][self.current_pos.y - 1].down = false;
                } else {
                    self.nodes[self.current_pos.x][self.current_pos.y].left = false;
                    self.nodes[self.current_pos.x - 1][self.current_pos.y].right = false;
                }
            } else if self.current_pos.x > 0 {
                self.nodes[self.current_pos.x][self.current_pos.y].left = false;
                self.nodes[self.current_pos.x - 1][self.current_pos.y].right = false;
            } else if self.current_pos.y > 0 {
                self.nodes[self.current_pos.x][self.current_pos.y].up = false;
                self.nodes[self.current_pos.x][self.current_pos.y - 1].down = false;
            }

            self.current_pos.x += 1;
            if self.current_pos.x >= GRID_WIDTH {
                self.current_pos.y += 1;
                self.current_pos.x = 0;
            }
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::BLACK);

        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                if (y > self.current_pos.y) || (y == self.current_pos.y && x > self.current_pos.x) {
                    break;
                }

                let node = &self.nodes[x][y];

                let screen_x = x as i32 * NODE_SIZE_I;
                let screen_y = y as i32 * NODE_SIZE_I;

                if y == self.current_pos.y {
                    d.draw_rectangle(screen_x, screen_y, NODE_SIZE_I, NODE_SIZE_I, Color::BLUE);
                } else {
                    d.draw_rectangle(screen_x, screen_y, NODE_SIZE_I, NODE_SIZE_I, Color::WHITE);
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

        d.draw_rectangle(
            self.current_pos.x as i32 * NODE_SIZE_I,
            self.current_pos.y as i32 * NODE_SIZE_I,
            NODE_SIZE_I,
            NODE_SIZE_I,
            Color::BLUE,
        );
    }
}
