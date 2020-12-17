use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

pub fn execute() {
    let file = File::open(".\\data\\13.txt").unwrap();
    let reader = BufReader::new(&file);
    let mut lines = reader.lines();

    let earliest_time_stamp:i32 = lines.next().unwrap().unwrap().parse().unwrap();
    let bus_ids:Vec<i32> = lines.next().unwrap().unwrap().split(',').filter(|c| { c.clone() != "x" }).map(|x| x.parse().unwrap()).collect();

    let bus_distances:Vec<(i32, i32)> = bus_ids.iter().map(|x| {
        (x.clone(), x - (earliest_time_stamp % x))
    }).collect();

    let (bus_id, distance) = bus_distances.iter().min_by(|(_, a), (_, b)| { a.cmp(b) }).unwrap();
    println!("{} {}", bus_id, distance);

    println!("{}", bus_id * distance);
}

