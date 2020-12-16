
use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

pub fn execute() {
    let file = File::open(".\\data\\12.txt").unwrap();
    let reader = BufReader::new(&file);

    let mut way_point = Pos { x: 10, y: -1}; // NS, EW
    let mut ship = Pos { x: 0, y: 0 };

    for line in reader.lines() {
        let l = line.unwrap();
        if l.len() > 0 {
            let instruction = l.chars().next().unwrap();
            let value_str:String = l.chars().skip(1).collect();
            let value:i32 = value_str.parse().unwrap();
            println!("{} {}", instruction, value);

            match instruction {
                'N' => way_point = way_point.move_me(0, -value),
                'S' => way_point = way_point.move_me(0, value),
                'E' => way_point = way_point.move_me(value, 0),
                'W' => way_point = way_point.move_me(-value, 0),
                'F' => {
                    for _ in 0..value {
                        ship = ship.move_me(way_point.x, way_point.y);
                    }
                },
                'L' => {
                    for _ in 0..(value / 90) {
                        way_point = way_point.rotate_left_90();
                    }
                },
                'R' => {
                    for _ in 0..(value / 90) {
                        way_point = way_point.rotate_right_90();
                    }
                }
                _ => panic!()
            }
        }
        println!("{:?} {:?}", ship, way_point);
    }

    // 61516

    println!("{}", ship.x.abs() + ship.y.abs());
}

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32
}

impl Pos {
    pub fn move_me(&self, delta_x:i32, delta_y:i32) -> Pos {
        Pos {
            x: self.x + delta_x,
            y: self.y + delta_y
        }
    }

    pub fn rotate_left_90(&self) -> Pos {
        Pos {
            x: self.y,
            y: -self.x
        }
    }

    pub fn rotate_right_90(&self) -> Pos {
        Pos {
            x: -self.y,
            y: self.x
        }
    }
}