
use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

pub fn execute() {
    let rules = vec![
        Rule { move_x: 1, move_y: 1 },
        Rule { move_x: 3, move_y: 1 },
        Rule { move_x: 5, move_y: 1 },
        Rule { move_x: 7, move_y: 1 },
        Rule { move_x: 1, move_y: 2 }
    ];

    let count:usize = rules.iter().map(|r| { process_lines(r) }).fold(1, |acc, x| { acc * x });

    println!("{}", count);
}

fn process_lines(rule:&Rule) -> usize {
    let file = File::open(".\\data\\03.txt").unwrap();
    let reader = BufReader::new(&file);

    let mut x = 0;
    let mut y = 0;
    let mut count = 0;

    for line in reader.lines() {
        if y == 0 {
            let l = line.unwrap();
            let line_len = l.len();
            let c = l.chars().skip(x).next().unwrap();
            if c == '#' {
                count += 1;
            }

            x += rule.move_x;
            if x >= line_len {
                x -= line_len;
            }
        }

        y += 1;
        if y >= rule.move_y {
            y = 0;
        }
    }

    count
}

struct Rule {
    move_x:usize,
    move_y:usize
}