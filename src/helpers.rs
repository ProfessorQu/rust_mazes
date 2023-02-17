use std::collections::HashSet;

use rand::{distributions::Standard, prelude::Distribution, seq::SliceRandom, thread_rng};

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

    pub fn get_neighbors(&self) -> Vec<Direction> {
        let mut neighbors = vec![];
        if self.x > 0 {
            let new_pos = Pos::new(self.x - 1, self.y);
            neighbors.push(Direction::Left(new_pos));
        }
        if self.x < GRID_WIDTH - 1 {
            let new_pos = Pos::new(self.x + 1, self.y);
            neighbors.push(Direction::Right(new_pos));
        }
        if self.y > 0 {
            let new_pos = Pos::new(self.x, self.y - 1);
            neighbors.push(Direction::Up(new_pos));
        }
        if self.y < GRID_HEIGHT - 1 {
            let new_pos = Pos::new(self.x, self.y + 1);
            neighbors.push(Direction::Down(new_pos));
        }

        neighbors
    }

    pub fn neighbors_not_in(&self, set: &HashSet<Pos>) -> Vec<Direction> {
        let mut neighbors = self.get_neighbors();
        neighbors.retain(|neighbor| !set.contains(&neighbor.get_pos()));
        neighbors
    }

    pub fn neighbors_in(&self, set: &HashSet<Pos>) -> Vec<Direction> {
        let mut neighbors = self.get_neighbors();
        neighbors.retain(|neighbor| set.contains(&neighbor.get_pos()));
        neighbors
    }

    pub fn get_random_neighbor(&self) -> Option<Direction> {
        self.get_neighbors().choose(&mut thread_rng()).copied()
    }

    pub fn get_random_neighbor_not_in(&self, set: &HashSet<Pos>) -> Option<Direction> {
        self.neighbors_not_in(set)
            .choose(&mut thread_rng())
            .copied()
    }

    pub fn get_random_neighbor_in(&self, set: &HashSet<Pos>) -> Option<Direction> {
        self.neighbors_in(set).choose(&mut thread_rng()).copied()
    }

    pub fn has_neighbors_in(&self, set: &HashSet<Pos>) -> bool {
        !self.neighbors_in(set).is_empty()
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

impl Distribution<Pos> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Pos {
        Pos::new(rng.gen_range(0..GRID_WIDTH), rng.gen_range(0..GRID_HEIGHT))
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

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct KruskalNode {
    pub up: bool,
    pub left: bool,
    pub set: i32,
}

impl KruskalNode {
    pub fn new(set: i32) -> Self {
        Self {
            up: true,
            left: true,
            set,
        }
    }
}
