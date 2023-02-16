use std::collections::HashSet;

use rand::{seq::SliceRandom, thread_rng};

use crate::{GRID_HEIGHT, GRID_WIDTH};

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up(Pos),
    Down(Pos),
    Left(Pos),
    Right(Pos),
}

impl Direction {
    pub fn get_pos(&self) -> Pos {
        match self {
            Direction::Down(x) => *x,
            Direction::Right(x) => *x,
            Direction::Left(x) => *x,
            Direction::Up(x) => *x,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn neighbors(&self, visited: &HashSet<Pos>) -> Vec<Direction> {
        let mut neighbors = vec![];
        if self.x > 0 {
            let new_pos = Pos::new(self.x - 1, self.y);
            if !visited.contains(&new_pos) {
                neighbors.push(Direction::Left(new_pos));
            }
        }
        if self.x < GRID_WIDTH - 1 {
            let new_pos = Pos::new(self.x + 1, self.y);
            if !visited.contains(&new_pos) {
                neighbors.push(Direction::Right(new_pos));
            }
        }
        if self.y > 0 {
            let new_pos = Pos::new(self.x, self.y - 1);
            if !visited.contains(&new_pos) {
                neighbors.push(Direction::Up(new_pos));
            }
        }
        if self.y < GRID_HEIGHT - 1 {
            let new_pos = Pos::new(self.x, self.y + 1);
            if !visited.contains(&new_pos) {
                neighbors.push(Direction::Down(new_pos));
            }
        }

        neighbors
    }

    pub fn get_random_neighbor(&self, visited: &HashSet<Pos>) -> Option<Direction> {
        self.neighbors(visited).choose(&mut thread_rng()).copied()
    }

    pub fn has_neighbors(&self, visited: &HashSet<Pos>) -> bool {
        if self.x > 0 {
            let new_pos = Pos::new(self.x - 1, self.y);
            if !visited.contains(&new_pos) {
                return true;
            }
        }
        if self.x < GRID_WIDTH - 1 {
            let new_pos = Pos::new(self.x + 1, self.y);
            if !visited.contains(&new_pos) {
                return true;
            }
        }
        if self.y > 0 {
            let new_pos = Pos::new(self.x, self.y - 1);
            if !visited.contains(&new_pos) {
                return true;
            }
        }
        if self.y < GRID_HEIGHT - 1 {
            let new_pos = Pos::new(self.x, self.y + 1);
            if !visited.contains(&new_pos) {
                return true;
            }
        }

        false
    }

    pub fn make_connection(&self, neighbor: &Direction, nodes: &mut [Vec<Node>]) {
        match neighbor {
            Direction::Left(_) => nodes[self.x][self.y].left = false,
            Direction::Right(next_pos) => nodes[next_pos.x][next_pos.y].left = false,
            Direction::Up(_) => nodes[self.x][self.y].up = false,
            Direction::Down(next_pos) => nodes[next_pos.x][next_pos.y].up = false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub up: bool,
    pub left: bool,
}

impl Node {
    pub fn new() -> Self {
        Self {
            up: true,
            left: true,
        }
    }
}
