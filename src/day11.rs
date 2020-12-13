
use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

pub fn execute() {
    let file = File::open(".\\data\\11.txt").unwrap();
    let reader = BufReader::new(&file);
    let mut width = 0;
    let mut data:Vec<char> = Vec::new();
    for line in reader.lines() {
        let l = line.unwrap();
        let len = l.len();
        if len > 0 {
            width = len;
            for c in l.chars() {
                data.push(c);
            }
        }
    }

    let mut grid = Grid {
        data: data,
        width: width
    };
    let mut done = false;

    while !done {
        let (new_grid, changed) = grid.iterate();
        done = !changed;
        grid = new_grid;
        let count = grid.count_seated();
        println!("{}", count);
    }

    println!("{}", done);
}

struct Grid {
    data:Vec<char>,
    width: usize
}

impl Grid {
    fn height(&self) -> usize { self.data.len() / self.width }

    fn pos_to_xy(&self, pos:usize) -> (usize, usize) {
        let y = pos / self.width;
        let x = pos - (y * self.width);
        (x, y)
    }

    fn xy_to_pos(&self, x:usize, y:usize) -> usize {
        (y * self.width) + x
    }

    fn seated_count(&self, x:usize, y:usize) -> i32 {
        match self.data.get(self.xy_to_pos(x, y)).unwrap() {
            '#' => 1,
            _ => 0
        }
    }

    fn move_point(&self, x:usize, y:usize, x_delta:i32, y_delta:i32) -> Option<(usize, usize)> {
        let new_x = if x_delta < 0 {
            if x == 0 {
                None
            } else {
                x.checked_sub(x_delta.abs() as usize)
            }
        } else { 
            match x.checked_add(x_delta as usize) {
                Some(x2) if x2 < self.width => Some(x2),
                _ => None
            }
        };
        let new_y = if y_delta < 0 {
            if y == 0 {
                None
            } else {
                y.checked_sub(y_delta.abs() as usize)
            }
        } else { 
            match y.checked_add(y_delta as usize) {
                Some(y2) if y2 < self.height() => Some(y2),
                _ => None
            }
         };

         if new_x.is_some() && new_y.is_some() {
            // println!("{} {} {} {}", x_delta, y_delta, new_y.unwrap(), new_y.unwrap());
            Some((new_x.unwrap(), new_y.unwrap()))
         } else {
            None
         }
    }

    fn scan_for_seat(&self, x:usize, y:usize, x_delta:i32, y_delta:i32) -> i32 {
        let new_point = self.move_point(x, y, x_delta, y_delta);
        match new_point {
            None => 0,
            Some((new_x, new_y)) => {
                match self.data[self.xy_to_pos(new_x, new_y)] {
                    '#' => 1,
                    '.' => self.scan_for_seat(new_x, new_y, x_delta, y_delta),
                    _ => 0
                }
            }
        }
    }

    fn adjacent_count(&self, x:usize, y:usize) -> i32 {
        let total =         self.scan_for_seat(x, y, -1, -1); // UL
        let total = total + self.scan_for_seat(x, y,  0, -1); // U
        let total = total + self.scan_for_seat(x, y,  1, -1); // UR
        let total = total + self.scan_for_seat(x, y,  1,  0); // R
        let total = total + self.scan_for_seat(x, y,  1,  1); // DR
        let total = total + self.scan_for_seat(x, y,  0,  1); // D
        let total = total + self.scan_for_seat(x, y, -1,  1); // DL
        let total = total + self.scan_for_seat(x, y, -1,  0); // L
        total
    }

    pub fn same(&self, other:&Grid) -> bool {
        self.data.eq(&other.data)
    }

    pub fn iterate(&self) -> (Grid, bool) {
        let mut new_data:Vec<char> = Vec::new();
        let mut changed = false;

        for p in 0..self.data.len() {
            let c = self.data[p];
            let (x, y) = self.pos_to_xy(p);
            let count = self.adjacent_count(x, y);

            let new_char = match c {
                'L' => {
                    if count == 0 { '#' } else { 'L' }
                },
                '#' => {
                    if count >= 5 { 'L' } else { '#' }
                },
                _ => '.'
            };
            
            changed = changed || new_char != c;
            new_data.push(new_char);
        }

        (Grid {
            data: new_data,
            width: self.width
        }, changed)
    }

    pub fn print(&self) {
        let mut x = 0;
        for c in self.data.iter() {
            print!("{}",c);
            x += 1;
            if x % self.width == 0 {
                println!();
            } 

        }
    }

    pub fn count_seated(&self) -> usize {
        self.data.iter().fold(0, |acc, c| { acc + (if *c == '#' { 1 } else { 0 }) })
    }
}