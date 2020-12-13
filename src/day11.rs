
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

    fn adjacent_count(&self, x:usize, y:usize) -> i32 {
        let total =         if y > 0 && x > 0              { self.seated_count(x - 1, y - 1) } else { 0 };
        let total = total + if y > 0                       { self.seated_count(x, y - 1) } else { 0 };
        let total = total + if y > 0 && x < self.width -1  { self.seated_count(x + 1, y - 1) } else { 0 };

        let total = total + if x > 0                       { self.seated_count(x - 1, y) } else { 0 };
        // skip self
        let total = total + if x < self.width - 1                          { self.seated_count(x + 1, y) } else { 0 };

        let total = total + if y < self.height() - 1 && x > 0              { self.seated_count(x - 1, y + 1) } else { 0 };
        let total = total + if y < self.height() - 1                       { self.seated_count(x, y + 1) } else { 0 };
        let total = total + if y < self.height() - 1 && x < self.width - 1 { self.seated_count(x + 1, y + 1) } else { 0 };

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
                    if count >= 4 { 'L' } else { '#' }
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