use std::{
    thread,
    time::{Duration, Instant},
};

use raylib::prelude::*;

use crate::Algorithm;

pub trait Maze {
    fn new() -> Self
    where
        Self: Sized;
    fn complete(&self) -> bool;
    fn reset(&mut self);
    fn generate(&mut self);
    fn draw(&self, d: &mut RaylibDrawHandle);
    fn update(
        &mut self,
        now: &mut Instant,
        current: &mut Algorithm,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        speed: usize,
    ) {
        if self.complete() {
            println!("{current:?} took {:?}", now.elapsed());

            thread::sleep(Duration::from_secs(2));
            self.reset();

            *current = current.next();
            *now = Instant::now();
        }

        let mut d = rl.begin_drawing(thread);

        for _ in 0..speed {
            self.generate();
        }

        self.draw(&mut d);
    }
}
