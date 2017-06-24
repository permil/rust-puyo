extern crate rand;

use piston_window::*;
use rand::Rng;

const WIDTH: usize = 6;
const HEIGHT: usize = 12;
const CEIL: usize = 2;

#[derive(PartialEq, Eq)]
enum State {
    Wait,
    Move,
}
impl Default for State {
    fn default() -> State { State::Wait }
}

#[derive(Default)]
pub struct Field {
    grids: [[usize; HEIGHT+CEIL+1]; WIDTH+2],
    x: usize,
    y: usize,
    kind: usize,
    state: State,
    down_count: usize
}
impl Field {
    pub fn new() -> Field {
        Field {
            grids: [[0; HEIGHT+CEIL+1]; WIDTH+2],
            ..Default::default()
        }
    }
    pub fn render<G>(&mut self, c: Context, g: &mut G)
            where G: Graphics {
        rectangle([0.5, 0.5, 0.5, 1.0],
                [32.0, 32.0, 32.0 * 6.0, 32.0 * 12.0],
                c.transform, g);
        for (i, column) in self.grids.iter().enumerate() {
            for (j, grid) in column.iter().enumerate() {
                if *grid != 0 {
                    ellipse(self.color(grid), // red
                            [(i * 32) as f64, ((j - CEIL) * 32) as f64, 32.0, 32.0],
                            c.transform, g);
                }
            }
        }
        if self.state == State::Move {
            ellipse(self.color(&self.kind),
                    [(self.x * 32) as f64, ((self.y - CEIL) * 32) as f64, 32.0, 32.0],
                    c.transform, g);
        }
    }
    fn color(&self, n: &usize) -> [f32; 4] {
        match *n {
            1 => [1.0, 0.0, 0.0, 1.0],
            2 => [0.0, 0.0, 1.0, 1.0],
            3 => [0.0, 1.0, 0.0, 1.0],
            4 => [1.0, 1.0, 0.0, 1.0],
            _ => [0.0, 0.0, 0.0, 0.0]
        }
    }
    pub fn right(&mut self) {
        if self.state != State::Move { return; }
        if self.x < WIDTH && self.grids[self.x+1][self.y] == 0 {
            self.x += 1;
        }
    }
    pub fn left(&mut self) {
        if self.state != State::Move { return; }
        if self.x > 1 && self.grids[self.x-1][self.y] == 0 {
            self.x -= 1;
        }
    }
    pub fn down(&mut self) {
        if self.state != State::Move { return; }
        if self.y < HEIGHT+CEIL && self.grids[self.x][self.y+1] == 0 {
            self.y += 1;
        } else {
            self.grids[self.x][self.y] = self.kind;
            self.state = State::Wait;
        }
        self.down_count = 0;
    }
    pub fn step(&mut self) {
        match self.state {
            State::Wait => {
                if !self.drop() {
                    if !self.erase() {
                        self.x = 3;
                        self.y = CEIL;
                        self.kind = rand::thread_rng().gen_range(1, 4);
                        self.state = State::Move;
                    }
                }
            },
            State::Move => {
                self.down_count += 1;
                if self.down_count >= 60 {
                    self.down();
                }
            }
        }
    }
    fn drop(&mut self) -> bool {
        let mut dropped = false;
        for i in 1..WIDTH+1 {
            let mut column = self.grids[i];
            for j in (CEIL..HEIGHT+CEIL).rev() {
                if self.grids[i][j] != 0 && self.grids[i][j+1] == 0 {
                    self.grids[i][j+1] = self.grids[i][j];
                    self.grids[i][j] = 0;
                    dropped = true;
                }
            }
        }
        dropped
    }
    fn erase(&mut self) -> bool {
        let mut erased = false;
        let mut flags = [[false; HEIGHT+CEIL+1]; WIDTH+2];
        for i in 1..WIDTH+1 {
            let mut column = self.grids[i];
            for j in CEIL..HEIGHT+CEIL+1 {
                if !flags[i][j] && self.grids[i][j] != 0 {
                    let connected = self.connect(&self.grids, &mut flags, i, j, self.grids[i][j]);
                    if connected.len() >= 4 {
                        for grid in &connected {
                            self.grids[grid.0][grid.1] = 0;
                        }
                        erased = true;
                    }
//                    print!("{} ", connected.len());
//                    for (ii, jj) in connected {
//                        print!("({}, {}) ", ii, jj);
//                    }
//                    println!("");
                }
            }
        }
        erased
    }
    fn connect(&self,
               grids: &[[usize; HEIGHT+CEIL+1]; WIDTH+2],
               flags: &mut [[bool; HEIGHT+CEIL+1]; WIDTH+2],
               x: usize,
               y: usize,
               color: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        if x <= 0 || x > WIDTH || y < CEIL || y > CEIL+HEIGHT {
            return result;
        }
        if flags[x][y] || self.grids[x][y] != color {
            return result;
        }

        flags[x][y] = true;
        result.push((x, y));
        result.append(&mut self.connect(grids, flags, x+1, y, color));
        result.append(&mut self.connect(grids, flags, x, y+1, color));
        result.append(&mut self.connect(grids, flags, x-1, y, color));
        result.append(&mut self.connect(grids, flags, x, y-1, color));
        return result
    }
}