
use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

pub fn execute() {
    let file = File::open(".\\data\\02.txt").unwrap();
    let reader = BufReader::new(&file);
    // .as_ref().unwrap().as_str() <- There's no way I'm doing this correctly
    let count = reader.lines().filter(|line| { test_line(line.as_ref().unwrap().as_str()) }).count();

    println!("{}", count);
}

struct PasswordRule {
    min_times: usize,
    max_times: usize,
    character: char
}

fn test_line(line: &str) -> bool {
    // "1-3 a"," abcde"
    let line_split:Vec<&str> = line.split(":").collect();

    // "1-3", "a"
    let rule_split:Vec<&str> = line_split[0].split(" ").collect();
    let numbers:Vec<usize> = rule_split[0].split("-").map(|x| { x.parse::<usize>().unwrap() }).collect();

    let rule = PasswordRule {
        character: rule_split[1].chars().next().unwrap(),
        min_times: numbers[0],
        max_times: numbers[1]
    };

    satisfies_rule(line_split[1], &rule)
}

fn satisfies_rule(password: &str, rule: &PasswordRule) -> bool {
    let count = password.chars().filter(|&c| { c == rule.character }).count();
    count >= rule.min_times && count <= rule.max_times
}