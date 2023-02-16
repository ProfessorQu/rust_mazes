use std::collections::HashSet;

use rand::{seq::IteratorRandom, thread_rng, Rng};
use raylib::prelude::*;

use crate::{helpers::*, maze::Maze, GRID_HEIGHT, GRID_WIDTH, NODE_SIZE_I};

#[derive(Clone)]
pub struct RandomizedPrim {
    nodes: Vec<Vec<Node>>,
    visited: HashSet<Pos>,
    edges: HashSet<Pos>,
}

impl RandomizedPrim {
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
        if next_pos.has_neighbors(&self.visited) {
            self.edges.insert(next_pos);
        }

        if !pos.has_neighbors(&self.visited) {
            self.edges.remove(&pos);
        }
    }
}

impl Maze for RandomizedPrim {
    fn new() -> Self {
        Self {
            nodes: vec![],
            visited: HashSet::new(),
            edges: HashSet::new(),
        }
    }

    fn complete(&self) -> bool {
        self.visited.len() == GRID_WIDTH * GRID_HEIGHT && self.edges.is_empty()
    }

    fn reset(&mut self) {
        self.nodes = vec![];
        for x in 0..GRID_WIDTH {
            self.nodes.push(vec![]);
            for _y in 0..GRID_HEIGHT {
                self.nodes[x].push(Node::new());
            }
        }

        self.visited = HashSet::new();
        self.edges = HashSet::new();

        let start_x = thread_rng().gen_range(0..GRID_WIDTH);
        let start_y = thread_rng().gen_range(0..GRID_HEIGHT);
        let start_pos = Pos::new(start_x, start_y);

        self.visited.insert(start_pos);
        self.edges.insert(start_pos);
    }

    fn generate(&mut self) {
        if let Some(pos) = self.edges.clone().iter().choose(&mut thread_rng()) {
            let neighbor = pos.get_random_neighbor(&self.visited);
            if let Some(neighbor) = neighbor {
                self.handle_neighbor(*pos, neighbor);
            } else {
                self.edges.remove(pos);
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
                    if self.edges.contains(&pos) {
                        d.draw_rectangle(
                            screen_x,
                            screen_y,
                            NODE_SIZE_I,
                            NODE_SIZE_I,
                            Color::PURPLE,
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
