use raylib::prelude::*;

pub trait Maze {
    fn new() -> Self;
    fn complete(&self) -> bool;
    fn reset(&mut self);
    fn generate(&mut self);
    fn draw(&self, d: &mut RaylibDrawHandle);
}
