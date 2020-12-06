
use std::collections::HashMap;

pub fn execute() {
    let file = std::fs::read_to_string(".\\data\\06.txt").unwrap();

    let item_count:usize = file.split("\n\n").map(|group| {
        let mut char_counts:HashMap<char, usize> = HashMap::new();
        let individuals = group.split("\n");
        let mut individual_count = 0;

        for individual in individuals.filter(|i| { i.len() > 0 }) {
            individual_count += 1;
            for c in individual.chars() {
                let new_count = match char_counts.remove(&c) {
                    None => 1,
                    Some(count) => count + 1
                };
                char_counts.insert(c, new_count);
            }
        }

        char_counts.into_iter().filter(|(_, v)| { v.clone() == individual_count }).count()
    }).sum();

    println!("{}", item_count);

}