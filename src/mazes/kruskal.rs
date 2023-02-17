use std::collections::HashSet;

use rand::{seq::SliceRandom, thread_rng};
use raylib::prelude::*;

use crate::{helpers::*, maze::Maze, GRID_HEIGHT, GRID_WIDTH, NODE_SIZE_I};

const LAST_POSSES_LEN: usize = 1100;

#[derive(Clone)]
pub struct Kruskal {
    nodes: Vec<Vec<KruskalNode>>,
    visited: HashSet<Pos>,
    unvisited: Vec<Pos>,
    all: Vec<Pos>,
    num_sets: usize,
    last_posses: Vec<Pos>,
}

impl Kruskal {
    fn handle_position(&mut self, pos: Pos) {
        let neighbor = pos.get_random_neighbor(&HashSet::new());
        if let Some(neighbor) = neighbor {
            let neighbor_pos = neighbor.get_pos();

            let this_set = self.nodes[pos.x][pos.y].set;
            let neighbor_set = self.nodes[neighbor_pos.x][neighbor_pos.y].set;

            if this_set != neighbor_set {
                self.num_sets -= 1;

                for x in 0..GRID_WIDTH {
                    for y in 0..GRID_HEIGHT {
                        if self.nodes[x][y].set == neighbor_set {
                            self.nodes[x][y].set = this_set;
                        }
                    }
                }

                self.handle_neighbor(pos, neighbor);
            }

            self.visited.insert(pos);
            self.visited.insert(neighbor_pos);

            self.last_posses.insert(0, pos);
            self.last_posses.insert(0, neighbor_pos);
            self.last_posses.truncate(LAST_POSSES_LEN);
        }
    }

    fn handle_neighbor(&mut self, pos: Pos, neighbor: Direction) {
        match neighbor {
            Direction::Left(_) => {
                self.nodes[pos.x][pos.y].left = false;
            }
            Direction::Right(next_pos) => {
                self.nodes[next_pos.x][next_pos.y].left = false;
            }
            Direction::Up(_) => {
                self.nodes[pos.x][pos.y].up = false;
            }
            Direction::Down(next_pos) => {
                self.nodes[next_pos.x][next_pos.y].up = false;
            }
        }
    }
}

impl Maze for Kruskal {
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
            unvisited: all.clone(),
            all,
            num_sets: GRID_WIDTH * GRID_HEIGHT,
            last_posses: vec![],
        }
    }

    fn complete(&self) -> bool {
        self.num_sets == 1
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

        self.all.clear();
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                self.all.push(Pos::new(x, y));
            }
        }

        self.all.shuffle(&mut thread_rng());
        self.unvisited = self.all.clone();
        self.num_sets = GRID_WIDTH * GRID_HEIGHT;
    }

    fn generate(&mut self) {
        if let Some(pos) = self.unvisited.pop() {
            self.handle_position(pos);
        } else if let Some(pos) = self.all.clone().choose(&mut thread_rng()) {
            self.handle_position(*pos);
        }

        if self.complete() {
            self.last_posses.clear();
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
                    if self.last_posses.contains(&pos) {
                        d.draw_rectangle(screen_x, screen_y, NODE_SIZE_I, NODE_SIZE_I, Color::RED);
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
