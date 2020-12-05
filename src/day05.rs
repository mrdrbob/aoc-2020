
use std::collections::HashSet;
use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

pub fn execute() {
    let file = File::open(".\\data\\05.txt").unwrap();
    let reader = BufReader::new(&file);
    let all_seats:HashSet<i32> = reader.lines().into_iter().map(|line| {
        calculate_seat_id(&line.unwrap())
    }).collect();

    let found = (2..2i32.pow(10)).find(|index| {
        all_seats.contains(&(index.clone() - 1)) && !all_seats.contains(index) && all_seats.contains(&(index.clone() + 1))
    });

    println!("{}", found.unwrap());
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