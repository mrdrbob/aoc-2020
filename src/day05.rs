
use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

pub fn execute() {
    let file = File::open(".\\data\\05.txt").unwrap();
    let reader = BufReader::new(&file);
    let result = reader.lines().into_iter().map(|line| {
        calculate_seat_id(&line.unwrap())
    }).max().unwrap();

    println!("{}", result);
}

fn calculate_seat_id(line:&String) -> i32 {
    let mut part1:String = line.chars().take(7).map(|c| {
        match c {
            'B' => '1',
            'F' => '0',
            _ => panic!()
        }
    }).collect();
    let part2:String = line.chars().skip(7).map(|c| {
        match c {
            'R' => '1',
            'L' => '0',
            _ => panic!()
        }
    }).collect();

    part1.push_str(&part2);
    i32::from_str_radix(&part1, 2).unwrap()
}