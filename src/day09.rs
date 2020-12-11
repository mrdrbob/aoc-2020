

use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

pub fn execute() {
    let file = File::open(".\\data\\09.txt").unwrap();
    let reader = BufReader::new(&file);

    let goal = 21806024;
    let numbers:Vec<usize> = reader.lines().map(|line| { line.unwrap().parse::<usize>().unwrap() })
        .collect();

    let mut start = 0;
    let mut end = 0;
    let mut sum = numbers[0];
    let mut done = false;

    while !done {
        if sum < goal {
            end += 1;
            sum += numbers[end];
        } else if sum > goal {
            sum -= numbers[start];
            start += 1;
        } else {
            done = true;
        }
    }

    println!("{}-{}", start, end);

    let mm_start:(Option<usize>, Option<usize>) = (None, None);

    let (mn, mx) = numbers[start..=end].iter().fold(mm_start, |(min, max), val| {
        println!("{} {:?} {:?}", val, min, max);
        let new_min = match min {
            Some(x) => if val.clone() < x { Some(val.clone()) } else { Some(x) },
            None => Some(val.clone())
        };
        let new_max = match max {
            Some(x) => if val.clone() > x { Some(val.clone()) } else { Some(x) },
            None => Some(val.clone())
        };
        (new_min, new_max)
    });

    println!("{} {}", mn.unwrap(), mx.unwrap());
    println!("{}", mn.unwrap() + mx.unwrap());

    /* Part 1
    let mut buffer:VecDeque<usize> = VecDeque::new();
    let mut numbers_index = 0usize;
    let buffer_size = 25;

    while buffer.len() < buffer_size {
        buffer.push_back(numbers[numbers_index]);
        numbers_index += 1;
    }

    while numbers_index < numbers.len() {
        let this_number = numbers.get(numbers_index).unwrap();
        let found_pair = buffer.iter().enumerate().any(|(index, number)| {
            buffer.iter().skip(index.clone() + 1).any(|n2| {
                n2.clone() + number.clone() == this_number.clone()
            })
        });
        if !found_pair {
            println!("{} {}", this_number, found_pair);
        }
        buffer.pop_front();
        buffer.push_back(this_number.clone());
        numbers_index = numbers_index + 1;
    }
    */
}

