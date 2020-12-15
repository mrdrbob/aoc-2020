
use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

pub fn execute() {
    let file = File::open(".\\data\\12.txt").unwrap();
    let reader = BufReader::new(&file);

    let mut state = (0, 0, 'E'); // NS, EW, Dir

    for line in reader.lines() {
        let l = line.unwrap();
        if (l.len() > 0) {
            let instruction = l.chars().next().unwrap();
            let value_str:String = l.chars().skip(1).collect();
            let value:i32 = value_str.parse().unwrap();
            let (x, y, d) = state;

            state = match instruction {
                'N' => (x, y - value, d),
                'S' => (x, y + value, d),
                'E' => (x + value, y, d),
                'W' => (x - value, y, d),
                'F' => match d {
                    'N' => (x, y - value, d),
                    'S' => (x, y + value, d),
                    'E' => (x + value, y, d),
                    'W' => (x - value, y, d),
                    _ => panic!()
                },
                'L' => match d {
                    'N' => match value {
                        90 => (x, y, 'W'),
                        180 => (x, y, 'S'),
                        270 => (x, y, 'E'),
                        _ => panic!()
                    },
                    'S' => match value {
                        90 => (x, y, 'E'),
                        180 => (x, y, 'N'),
                        270 => (x, y, 'W'),
                        _ => panic!()
                    },
                    'E' => match value {
                        90 => (x, y, 'N'),
                        180 => (x, y, 'W'),
                        270 => (x, y, 'S'),
                        _ => panic!()
                    },
                    'W' => match value {
                        90 => (x, y, 'S'),
                        180 => (x, y, 'E'),
                        270 => (x, y, 'N'),
                        _ => panic!()
                    },
                    _ => panic!()
                },
                'R' => match d {
                    'N' => match value {
                        90 => (x, y, 'E'),
                        180 => (x, y, 'S'),
                        270 => (x, y, 'W'),
                        _ => panic!()
                    },
                    'S' => match value {
                        90 => (x, y, 'W'),
                        180 => (x, y, 'N'),
                        270 => (x, y, 'E'),
                        _ => panic!()
                    },
                    'E' => match value {
                        90 => (x, y, 'S'),
                        180 => (x, y, 'W'),
                        270 => (x, y, 'N'),
                        _ => panic!()
                    },
                    'W' => match value {
                        90 => (x, y, 'N'),
                        180 => (x, y, 'E'),
                        270 => (x, y, 'S'),
                        _ => panic!()
                    },
                    _ => panic!()
                }
                _ => panic!()
            }
        }
        println!("{:?}", state);
    }

    let (x, y, _) = state;
    println!("{}", x.abs() + y.abs());
}
