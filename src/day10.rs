
use std::collections::HashMap;
use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

pub fn execute() {
    let file = File::open(".\\data\\10.txt").unwrap();
    let reader = BufReader::new(&file);
    let mut numbers:Vec<usize> = reader.lines().map(|line| { line.unwrap().parse::<usize>().unwrap() }).collect();
    let mut possibilities:HashMap<usize, usize> = HashMap::new();

    // Need to add the socket, since in my input there are two adapters that can connect to the socket.
    numbers.push(0);
    numbers.sort();
    let mut total:usize = 0;
    
    // In order to prime the lookup of possible paths, I'm going to insert a 1 into the map here,
    // and then just skip this jolt while iterating through the list.
    let final_jolt = numbers.iter().last().unwrap();
    possibilities.insert(*final_jolt, 1);
    println!("{}", final_jolt);

    for x in (0..numbers.len() - 1).rev() {
        let current_jolt = numbers[x];

        let possible_jumps = numbers.iter().skip(x + 1).take(4).filter(|jolt| { jolt.checked_sub(current_jolt).unwrap() <= 3 });
        let total_possible_jumps = possible_jumps.fold(0, |acc, jolt_number| { acc + possibilities.get(jolt_number).unwrap().clone()  });
        possibilities.insert(current_jolt, total_possible_jumps);
        total = total_possible_jumps;
        println!("{} {}", current_jolt, total);
    }

    println!("{}", total);

    /* Part 1
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
    */
}
