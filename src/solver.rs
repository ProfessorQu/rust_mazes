use std::collections::HashSet;

use raylib::prelude::*;

use crate::helpers::*;
use crate::maze::Maze;
use crate::*;

pub struct Solver {
    stack: Vec<Pos>,
    visited: HashSet<Pos>,
    end_pos: Pos,
    maze: Maze,
}

impl Solver {
    pub fn new(maze: Maze, start_pos: Pos, end_pos: Pos) -> Self {
        Self {
            stack: vec![start_pos],
            visited: HashSet::from_iter(vec![start_pos]),
            end_pos,
            maze,
        }
    }

    pub fn set_maze(&mut self, maze: Maze) {
        self.maze = maze;
    }

    fn handle_neighbor(&mut self, pos: Pos, neighbor: Direction) -> bool {
        let next_pos = match neighbor {
            Direction::Left(next_pos) => {
                if next_pos == self.end_pos {
                    return true;
                }
                next_pos
            }
            Direction::Right(next_pos) => {
                if next_pos == self.end_pos {
                    return true;
                }
                next_pos
            }
            Direction::Up(next_pos) => {
                if next_pos == self.end_pos {
                    return true;
                }
                next_pos
            }
            Direction::Down(next_pos) => {
                if next_pos == self.end_pos {
                    return true;
                }
                next_pos
            }
        };

        self.visited.insert(next_pos);

        self.stack.push(pos);
        self.stack.push(next_pos);

        false
    }

    pub fn solve(&mut self) -> bool {
        if let Some(pos) = self.stack.pop() {
            let neighbor = pos.get_neighbor(&self.visited);
            if let Some(neighbor) = neighbor {
                return self.handle_neighbor(pos, neighbor);
            }
        }

        false
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for pos in &self.stack {
            let x = pos.x as i32 * NODE_SIZE_I;
            let y = pos.y as i32 * NODE_SIZE_I;
            d.draw_rectangle(x, y, NODE_SIZE_I, NODE_SIZE_I, Color::GREEN);
        }
    }
}
