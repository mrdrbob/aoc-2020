
use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

pub fn execute() {
    let file = File::open(".\\data\\03.txt").unwrap();
    let reader = BufReader::new(&file);

    let mut x = 0;
    let mut count = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        let line_len = l.len();
        if x >= line_len {
            x -= line_len;
        }
        let c = l.chars().skip(x).next().unwrap();
        if c == '#' {
            count += 1;
        }
        x += 3;
    }

    println!("{}", count);
}