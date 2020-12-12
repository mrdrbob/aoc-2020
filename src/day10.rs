
use std::collections::HashMap;
use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

pub fn execute() {
    let file = File::open(".\\data\\10.txt").unwrap();
    let reader = BufReader::new(&file);
    let mut numbers:Vec<i32> = reader.lines().map(|line| { line.unwrap().parse::<i32>().unwrap() }).collect();
    numbers.sort();

    let mut last_number:i32 = 0;
    let mut distribution:HashMap<i32, i32> = HashMap::new();
    
    for number in numbers {
        let diff = number - last_number;
        if diff > 3 {
            println!("Too big diff");
        }
        let new_dist_value = match distribution.remove(&diff) {
            None => 1,
            Some(x) => x + 1
        };
        distribution.insert(diff, new_dist_value);
        last_number = number;
    }


    println!("{:?}", distribution);
    let one_jolt = distribution.get(&1).unwrap();
    let three_jolt = distribution.get(&3).unwrap() + 1; // plus 1 for the device

    println!("{}, {}, {}", one_jolt, three_jolt, one_jolt * three_jolt);
}
