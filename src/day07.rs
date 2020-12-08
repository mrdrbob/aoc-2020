

use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

pub fn execute() {
    let file = File::open(".\\data\\07.txt").unwrap();
    let reader = BufReader::new(&file);

    let mut bag_list:Vec<Bag> = Vec::new();

    for line in reader.lines() {
        let l = line.unwrap();
        parse_bag(&mut bag_list, l.as_str());
    }

    let names:Vec<String> = bag_list.iter().map(|bag| { bag.name.clone() }).collect();
    let count = names.iter().filter(|bag_name| { bag_can_contain(&bag_list, bag_name, "shiny gold")  }).count();
    println!("{}", count);
}

fn bag_can_contain(bag_list:&Vec<Bag>, bag_name:&str, name:&str) -> bool {
    let bag = bag_list.iter().find(|b| { b.name == bag_name  }).unwrap();

    let can_contain_directly = bag.contains.iter().any(|child| { child.bag_id == name });
    if can_contain_directly {
        true
    } else {
        let any_child_bag_can_contain = bag.contains.iter().any(|child| {
            // let child_bag = bag_list.iter().find(|cb| { cb.name  == child.bag_id }).unwrap();
            bag_can_contain(bag_list, child.bag_id.as_str(), name)
        });

        any_child_bag_can_contain
    }


}



fn parse_bag(bag_list:&mut Vec<Bag>, line:&str) {
    // dark olive bags contain 2 muted brown bags, 1 mirrored tomato bag, 4 bright black bags.
    let mut split = line.split(" bags contain ");
    let name = split.next().unwrap().to_owned();
    let count_line = split.next().unwrap();
    let mut bag_counts:Vec<BagChild> = Vec::new();

    if count_line != "no other bags." {
        for bag_count in count_line.split(", ") {
            let len = bag_count.len();
            let count:i32 = bag_count[0..1].parse().unwrap();
            let mut end_index = len - (if count == 1 { 4 } else { 5 });
            if &bag_count[(len - 1)..len] == "." {
                end_index -= 1;
            }
            let child_name = bag_count[2..end_index].to_owned();
            let child = BagChild { 
                count: count,
                bag_id: child_name
            };
            bag_counts.push(child);
        }
    }

    let b = Bag{ 
        name: name,
        contains: bag_counts
    };
    bag_list.push(b);
}

struct Bag {
    name: String,
    contains: Vec<BagChild>
}

struct BagChild {
    count: i32,
    bag_id: String
}
