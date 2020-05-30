use rand::Rng;
use std::{thread, time};
use termion::{color, style};

#[derive(Debug)]
struct Live {
    cells: Vec<Vec<bool>>,
    xlen: usize,
    ylen: usize
}

impl Live {
    fn new(x: usize, y: usize) -> Live {
        Live {
            cells: vec![vec![false; y]; x],
            xlen: x,
            ylen: y
        }
    }

    fn draw(&self) {
        print!("{}", termion::cursor::Goto(1, 1));
        for y in &self.cells {
            for x in y {
                print!("{}", if *x
                {format!("{}*{}", color::Fg(color::Green), style::Reset)}
                else
                {format!("{}.{}", color::Fg(color::LightBlack), style::Reset)});
            }
            println!();
        }
    }

    fn fill_rnd(&mut self, p: usize) {
        for y in self.cells.iter_mut() {
            for x in y {
                *x = if p < rand::thread_rng().gen_range(1, 100) {
                    false
                } else {
                    true
                }
            }
        }
    }

    fn check(&self, x: usize, y: usize) -> usize {
        if self.cells[x][y] {1} else {0}
    }

    fn neighbours_count(&self, x: usize, y: usize) -> usize {
        let xlen = self.xlen;
        let ylen = self.ylen;
        let mut count = 0;

        count += self.check((x+xlen-1) % xlen, (y+ylen-1) % ylen);
        count += self.check((x+xlen) % xlen, (y+ylen-1) % ylen);
        count += self.check((x+xlen+1) % xlen, (y+ylen-1) % ylen);
        count += self.check((x+xlen-1) % xlen, (y+ylen) % ylen);
        count += self.check((x+xlen+1) % xlen, (y+ylen) % ylen);
        count += self.check((x+xlen-1) % xlen, (y+ylen+1) % ylen);
        count += self.check((x+xlen) % xlen, (y+ylen+1) % ylen);
        count += self.check((x+xlen+1) % xlen, (y+ylen+1) % ylen);

        count
    }

    fn update(&mut self) {
        let mut next_cells = self.cells.clone();
        for x in 0..self.xlen {
            for y in 0..self.ylen {
                let neighbours = self.neighbours_count(x, y);
                match neighbours {
                    3       => next_cells[x][y] = true,
                    2       => continue,
                    _       => next_cells[x][y] = false,
                }
            }
        }
        self.cells = next_cells;
    }

    fn run(&mut self) {
        let timeout = time::Duration::from_millis(100);
        loop {
            self.draw();
            self.update();
            thread::sleep(timeout);
        }
    }
}

fn main() {
    let mut live = Live::new(50, 100);
    live.fill_rnd(10);
    live.run();
}
