mod live {
    use rand::Rng;
    use std::{thread, time, fmt};
    use termion::{color, style, cursor};

    #[derive(Debug)]
    pub struct LiveObject {
        cells: Vec<Vec<bool>>,
        xlen: usize,
        ylen: usize
    }

    impl fmt::Display for LiveObject {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut out = String::from(cursor::Goto(1, 1));
            for y in self.cells.iter() {
                for x in y {
                    if *x {
                        out.push_str(&format!("{}*{}", color::Fg(color::Green), style::Reset));
                    } else {
                        out.push_str(&format!("{}.{}", color::Fg(color::LightBlack), style::Reset));
                    }
                }
                out.push_str("\n");
            }
            write!(f, "{}", out)
        }
    }

    impl LiveObject {
        pub fn new(x: usize, y: usize, p: usize) -> LiveObject {
            LiveObject::fill_rnd(
                LiveObject {
                cells: vec![vec![false; y]; x],
                xlen: x,
                ylen: y,
            }, p)
        }

        fn draw(&self) {
            println!("{}", self.to_string());
        }

        fn fill_rnd(mut live: LiveObject, p: usize) -> LiveObject {
            for y in live.cells.iter_mut() {
                for x in y {
                    *x = if p < rand::thread_rng().gen_range(1, 100) {
                        false
                    } else {
                        true
                    }
                }
            }
            live
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
                        3 => next_cells[x][y] = true,
                        2 => continue,
                        _ => next_cells[x][y] = false,
                    }
                }
            }
            self.cells = next_cells;
        }

        pub fn run(&mut self) {
            let timeout = time::Duration::from_millis(100);
            loop {
                self.draw();
                self.update();
                thread::sleep(timeout);
            }
        }
    }
}


fn main() {
    let mut live = live::LiveObject::new(50, 100, 10);
    live.run();
}
