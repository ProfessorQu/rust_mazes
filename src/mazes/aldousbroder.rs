use std::collections::HashSet;

use rand::{thread_rng, Rng};
use raylib::prelude::*;

use crate::{helpers::*, maze::Maze, GRID_HEIGHT, GRID_WIDTH, NODE_SIZE_I};

const LAST_POSSES_LEN: usize = 10000;

#[derive(Clone)]
pub struct AldousBroder {
    nodes: Vec<Vec<Node>>,
    visited: HashSet<Pos>,
    current_pos: Pos,
    last_posses: Vec<Pos>,
}

impl Maze for AldousBroder {
    fn new() -> Self {
        Self {
            nodes: vec![],
            visited: HashSet::new(),
            current_pos: Pos::new(0, 0),
            last_posses: vec![],
        }
    }

    fn complete(&self) -> bool {
        self.visited.len() == GRID_WIDTH * GRID_HEIGHT
    }

    fn reset(&mut self) {
        self.nodes.clear();
        for x in 0..GRID_WIDTH {
            self.nodes.push(vec![]);
            for _y in 0..GRID_HEIGHT {
                self.nodes[x].push(Node::new());
            }
        }

        self.visited.clear();

        let start_x = thread_rng().gen_range(0..GRID_WIDTH);
        let start_y = thread_rng().gen_range(0..GRID_HEIGHT);
        let start_pos = Pos::new(start_x, start_y);

        self.visited.insert(start_pos);
        self.current_pos = start_pos;

        self.last_posses.clear();
    }

    fn generate(&mut self) {
        if !self.complete() {
            if let Some(neighbor) = self.current_pos.get_random_neighbor(&HashSet::new()) {
                let neighbor_pos = neighbor.get_pos();
                if !self.visited.contains(&neighbor_pos) {
                    self.current_pos.make_connection(&neighbor, &mut self.nodes);
                }

                self.current_pos = neighbor_pos;
                self.visited.insert(neighbor_pos);

                self.last_posses.insert(0, neighbor_pos);
                self.last_posses.truncate(LAST_POSSES_LEN);
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
                    d.draw_rectangle(screen_x, screen_y, NODE_SIZE_I, NODE_SIZE_I, Color::WHITE);

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

        if !self.complete() {
            for (i, pos) in self.last_posses.iter().enumerate() {
                let mut color = Color::LIGHTGRAY;
                color.r *= i as u8 / LAST_POSSES_LEN as u8;
                color.g *= i as u8 / LAST_POSSES_LEN as u8;
                color.b *= i as u8 / LAST_POSSES_LEN as u8;

                d.draw_rectangle(
                    pos.x as i32 * NODE_SIZE_I,
                    pos.y as i32 * NODE_SIZE_I,
                    NODE_SIZE_I,
                    NODE_SIZE_I,
                    color,
                );
            }
        }
    }
}
