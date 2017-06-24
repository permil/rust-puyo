use piston_window::*;

const WIDTH: usize = 6;
const HEIGHT: usize = 12;
const CEIL: usize = 2;

#[derive(Default)]
pub struct Field {
    grids: [[usize; 15]; 8],
    x: usize,
    y: usize,
    drop_count: usize
}
impl Field {
    pub fn new() -> Field {
        Field {
            grids: [[0; HEIGHT+CEIL+1]; WIDTH+2],
            x: 3,
            y: CEIL,
            ..Default::default()
        }
    }
    pub fn render<G>(&mut self, c: Context, g: &mut G)
            where G: Graphics {
        rectangle([0.5, 0.5, 0.5, 1.0],
                [32.0, 32.0, 32.0 * 6.0, 32.0 * 12.0],
                c.transform, g);
        for (i, row) in self.grids.iter().enumerate() {
            for (j, grid) in row.iter().enumerate() {
                if *grid != 0 {
                    ellipse([1.0, 0.0, 0.0, 1.0], // red
                            [(i * 32) as f64, ((j - CEIL) * 32) as f64, 32.0, 32.0],
                            c.transform, g);
                }
            }
        }
        ellipse([1.0, 0.0, 0.0, 1.0], // red
                [(self.x * 32) as f64, ((self.y - CEIL) * 32) as f64, 32.0, 32.0],
                c.transform, g);
    }
    pub fn right(&mut self) {
        if self.x < WIDTH && self.grids[self.x+1][self.y] == 0 {
            self.x += 1;
        }
    }
    pub fn left(&mut self) {
        if self.x > 1 && self.grids[self.x-1][self.y] == 0 {
            self.x -= 1;
        }
    }
    pub fn down(&mut self) {
        if self.y < HEIGHT+CEIL && self.grids[self.x][self.y+1] == 0 {
            self.y += 1;
        }
    }
    pub fn step(&mut self) {
        self.drop_count += 1;
        if self.drop_count >= 60 {
            if self.y < HEIGHT+CEIL && self.grids[self.x][self.y+1] == 0 {
                self.y += 1;
            } else {
                self.grids[self.x][self.y] = 1;
                self.x = 3;
                self.y = CEIL;
            }
            self.drop_count = 0;
        }
    }
}