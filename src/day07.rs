

use core::convert::Infallible;
use core::str::FromStr;
use std::collections::HashMap;
use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

pub fn execute() {
    let file = File::open(".\\data\\07.txt").unwrap();
    let reader = BufReader::new(&file);

    let mut bag_container:BagContainer = BagContainer::new();

    for line in reader.lines() {
        let l = line.unwrap();
        let bag:Bag = l.parse().unwrap();
        bag_container.push(bag);
    }

    // let names:Vec<String> = bag_container.all_bags.iter().map(|(_, bag)| { bag.name.clone() }).collect();
    // let count = names.iter().filter(|bag_name| { bag_container.can_bag_contain(bag_name, "shiny gold")  }).count();
    // println!("{}", count);

    let count = bag_container.count("shiny gold");
    println!("{}", count);
}

struct BagContainer {
    all_bags: HashMap<String, Bag>
}

impl BagContainer {
    fn new() -> BagContainer {
        BagContainer {
            all_bags: HashMap::new()
        }
    }

    fn push(&mut self, bag:Bag) {
        self.all_bags.insert(bag.name.clone(), bag);
    }

    fn count(&self, name:&str) -> i32  {
        let bag = self.all_bags.get(name).unwrap();
        bag.contains.iter().fold(0i32, |acc, item| {
            let child = self.all_bags.get(&item.bag_id).unwrap();
            acc + item.count + (item.count * self.count(&child.name))
        })
    }

    fn can_bag_contain(&self, bag_name:&str, name:&str) -> bool {
        let bag = self.all_bags.get(bag_name).unwrap();

        let can_contain_directly = bag.contains.iter().any(|child| { child.bag_id == name });
        if can_contain_directly {
            true
        } else {
            let any_child_bag_can_contain = bag.contains.iter().any(|child| {
                self.can_bag_contain(child.bag_id.as_str(), name)
            });
    
            any_child_bag_can_contain
        }
    
    }
}

struct Bag {
    name: String,
    contains: Vec<BagChild>
}

impl FromStr for Bag {
    type Err = Infallible;
    
    fn from_str(line: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
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
    
        Ok(Bag { 
            name: name,
            contains: bag_counts
        })
    }
}

struct BagChild {
    count: i32,
    bag_id: String
}
