

use core::str::FromStr;
use core::convert::Infallible;
use std::collections::HashSet;

pub fn execute() {
    let file = std::fs::read_to_string(".\\data\\16.txt").unwrap();

    let sections:Vec<&str> = file.split("\n\n").collect();
    let rules:Vec<Rule> = sections[0].split("\n").map(|line| line.parse().unwrap()).collect();
    let tickets:Vec<Vec<i32>> = sections[2].split("\n").skip(1)
        .filter(|line| line.len() > 0)
        .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect()).collect();
    
    let mut allValidNumbers:HashSet<i32> = HashSet::new();
    for rule in rules.iter() {
        for n in rule.first_lower..rule.first_upper {
            allValidNumbers.insert(n);
        }
        for n in rule.second_lower..rule.second_upper {
            allValidNumbers.insert(n);
        }
    }

    let mut sum:i32 = 0;
    for ticket in tickets {
        for num in ticket {
            if !allValidNumbers.contains(&num) {
                sum += num;
            }
        }
    }

    println!("{}", sum);
}

struct Rule {
    field_name:String,
    first_lower: i32,
    first_upper: i32,
    second_lower: i32,
    second_upper: i32
}

impl FromStr for Rule {
    type Err = Infallible;

    
    fn from_str(line: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> { 

        // zone: 42-668 or 688-958
        let mut split = line.split(": ");
        let name = split.next().unwrap();

        // 42-668 or 688-958
        let mut ranges = split.next().unwrap().split(" or ");
        let mut first_range = ranges.next().unwrap().split("-");
        let mut second_range = ranges.next().unwrap().split("-");

        // So elegant...
        let first_lower:i32 = first_range.next().unwrap().parse().unwrap();
        let first_upper:i32 = first_range.next().unwrap().parse().unwrap();

        let second_lower:i32 = second_range.next().unwrap().parse().unwrap();
        let second_upper:i32 = second_range.next().unwrap().parse().unwrap();

        Ok(Rule {
            field_name: name.to_owned(),
            first_lower: first_lower,
            first_upper: first_upper,
            second_lower: second_lower,
            second_upper: second_upper
        })
     }
}