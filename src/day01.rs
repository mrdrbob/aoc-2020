use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;

pub fn execute() {
    let file = File::open(".\\data\\01.txt").unwrap();
    let reader = BufReader::new(&file);
    
    let all_lines:HashSet<i32> = reader.lines().map(|item| { item.unwrap().parse::<i32>().unwrap() }).collect();
    let t = all_lines.iter().find(|x| all_lines.contains(&(2020 - x.clone()))).unwrap();
    let result = t * (2020 - t);
    println!("{}", result);
}
