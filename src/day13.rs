use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

enum ScheduleSlot {
    Bus (usize, usize),
    Empty
}


pub fn execute() {
    let file = File::open(".\\data\\13.txt").unwrap();
    let reader = BufReader::new(&file);
    let mut lines = reader.lines();

    let _earliest_time_stamp = lines.next().unwrap();
    let slots:Vec<ScheduleSlot> = lines.next().unwrap().unwrap().split(',').enumerate().map(|(index, text)| {
        match text {
            "x" => ScheduleSlot::Empty,
            _ => ScheduleSlot::Bus(text.parse().unwrap(), index)
        }
    }).collect();

    let mut offset:usize = 100000000000000;
    let mut done = false;
    let total_slots = slots.len();

    while !done {
        // Figure out which slots work for this particular time offset.
        // x slots count as freebies
        let matching_slots = slots.iter().filter(|bus| match bus {
            ScheduleSlot::Empty => true,
            ScheduleSlot::Bus (id, index) => (offset + index) % id == 0
        });

        // Count the total number that work for this slot.
        let total_matches = matching_slots.clone().count();
        println!("{}", total_matches);
        
        // If it's all the slots, we've found it!
        if total_matches == total_slots {
            done = true;
        } else {
            // Not all work. We can't iterate 1 at a time, that will take too long. So what's the next possible offset?
            // Need to find the next slot that is going to work for the offsets. The offsets are constant, so I only have to worry
            // about the relative jump here. So I need to find a relative jump that works for all the ids that are currently matching.
            // Luckily, all the bus ids are prime, so I don't have to find common factors. I just multiply them all together. That 
            // will give me the next best offset to check, and allows me to skip a ton of offsets.
            let next_offset = matching_slots.fold(1usize, |acc, slot| {
                match slot {
                    ScheduleSlot::Empty => acc,
                    ScheduleSlot::Bus (id, _) => acc * id
                }
            });
            // Jump ahead. The next offset will match at least as many slots as this one.
            offset += next_offset;
        }
    }

    println!("Offset: {}", offset);

    /*
    // part 1
    let earliest_time_stamp:i32 = lines.next().unwrap().unwrap().parse().unwrap();
    let bus_ids:Vec<i32> = lines.next().unwrap().unwrap().split(',').filter(|c| { c.clone() != "x" }).map(|x| x.parse().unwrap()).collect();

    let bus_distances:Vec<(i32, i32)> = bus_ids.iter().map(|x| {
        (x.clone(), x - (earliest_time_stamp % x))
    }).collect();

    let (bus_id, distance) = bus_distances.iter().min_by(|(_, a), (_, b)| { a.cmp(b) }).unwrap();
    println!("{} {}", bus_id, distance);

    println!("{}", bus_id * distance);
    */
}

