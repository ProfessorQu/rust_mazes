use std::collections::HashSet;

use rand::{seq::SliceRandom, thread_rng};
use raylib::prelude::*;

use crate::{helpers::*, maze::Maze, GRID_HEIGHT, GRID_WIDTH, NODE_SIZE_I};

const LAST_POSSES_LEN: usize = 1100;

#[derive(Clone)]
pub struct Wilson {
    nodes: Vec<Vec<KruskalNode>>,
    visited: HashSet<Pos>,
    unvisited: Vec<Pos>,
    stack: Vec<Direction>,
    start_generate_pos: Pos,
    generate_pos: Pos,
    found_path: bool,
}

impl Wilson {}

impl Maze for Wilson {
    fn new() -> Self {
        let mut all = vec![];
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                all.push(Pos::new(x, y));
            }
        }

        all.shuffle(&mut thread_rng());

        Self {
            nodes: vec![],
            visited: HashSet::new(),
            unvisited: all,
            stack: vec![],
            start_generate_pos: Pos::new(0, 0),
            generate_pos: Pos::new(0, 0),
            found_path: false,
        }
    }

    fn complete(&self) -> bool {
        self.visited.len() == GRID_WIDTH * GRID_HEIGHT
    }

    fn reset(&mut self) {
        let mut set = 0;
        self.nodes.clear();
        for x in 0..GRID_WIDTH {
            self.nodes.push(vec![]);
            for _y in 0..GRID_HEIGHT {
                self.nodes[x].push(KruskalNode::new(set));
                set += 1;
            }
        }

        self.visited.clear();

        let mut all = vec![];
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                all.push(Pos::new(x, y));
            }
        }

        all.shuffle(&mut thread_rng());

        self.start_generate_pos = all[0];
        self.generate_pos = self.start_generate_pos;
        self.unvisited = all;

        self.found_path = false;
    }

    fn generate(&mut self) {
        if !self.found_path {
            if self.generate_pos.has_neighbors_in(&self.visited) {
                let dir = self
                    .generate_pos
                    .get_random_neighbor_in(&self.visited)
                    .expect("Failed to get visited");

                self.stack.push(dir);

                self.found_path = true;
            } else {
                let random_dir = self
                    .generate_pos
                    .get_random_neighbor()
                    .expect("Failed to get a random neighbor");

                self.stack.push(random_dir);
            }
        }
        else {

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
    }
}
