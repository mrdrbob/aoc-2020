use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::vec::Vec;

pub fn execute() {
    let file = File::open(".\\data\\01.txt").unwrap();
    let reader = BufReader::new(&file);
    
    /* Part 1 solve
    let all_lines:HashSet<i32> = reader.lines().map(|item| { item.unwrap().parse::<i32>().unwrap() }).collect();
    let t = all_lines.iter().find(|x| all_lines.contains(&(2020 - x.clone()))).unwrap();
    let result = t * (2020 - t);
    println!("{}", result);
    */

    /* Part 2 solve */
    let all_lines:Vec<i32> = reader.lines().map(|item| { item.unwrap().parse::<i32>().unwrap() }).collect();
    let combo = find_combo_that_adds_up(&all_lines, 2020).unwrap();
    println!("{}", combo);
}

pub fn find_combo_that_adds_up(all: &Vec<i32>, looking_for:i32) -> Option<i32> {
    for (i1, val1) in all.iter().enumerate() {
        for (i2, val2) in all.iter().enumerate().filter(|&(i, _)| i != i1) {
            for (_, val3) in all.iter().enumerate().filter(|&(i, _)| i != i1 && i != i2) {
                if val1 + val2 + val3 == looking_for {
                    // Probably the least "rusty" way to do this.
                    return Some(val1 * val2 * val3)
                }
            }
        }
    }

    None
}