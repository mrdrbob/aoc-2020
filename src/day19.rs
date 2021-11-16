use std::collections::HashMap;
use core::str::FromStr;
use core::convert::Infallible;

pub fn execute() {
    let file = std::fs::read_to_string(".\\data\\19.txt").unwrap();
    let split:Vec<&str> = file.split("\n\n").collect();
    
    let rules_list:Vec<Rule> = split[0].lines().map(|line| line.parse().unwrap()).collect();
    let rules = RuleList::new(rules_list);
    
    let match_count:usize = split[1].lines().map(|line| {
        let t:Vec<char> = line.chars().collect();
        let is_match = rules.check_match(&t);
        if is_match { 1 } else { 0 }
    }).sum();


    println!("{}", match_count);
}

struct Rule {
    id: usize,
    implementation: RuleImplementation
}

enum RuleImplementation {
    Character(char),
    Rule(usize),
    Any(Vec<RuleImplementation>),
    Each(Vec<RuleImplementation>),
    End
}

impl FromStr for Rule {
    type Err = Infallible;

    fn from_str(line: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> { 
        let split:Vec<&str> = line.split(": ").collect();
        let rule_id = split[0].parse::<usize>().unwrap();
        let implementation = if split[1].chars().next().unwrap() == '"' {
            let character = split[1].chars().skip(1).next().unwrap();
            RuleImplementation::Character(character)
        } else {
            RuleImplementation::Any(split[1].split(" | ").map(|group| {
                RuleImplementation::Each(group.split(" ").map(|id| {  
                    RuleImplementation::Rule(id.parse::<usize>().unwrap())
                }).collect())
            }).collect())
        };

        Ok(Rule {
            id: rule_id,
            implementation: implementation
        })
    }
}

struct RuleList {
    rules:HashMap<usize, Rule>
}

impl RuleList {
    fn new(rules:Vec<Rule>) -> RuleList {
        let mut rules_map:HashMap<usize,Rule> = HashMap::new();

        for rule in rules {
            rules_map.insert(rule.id, rule);
        }

        RuleList { rules: rules_map }
    }

    fn check_match(&self, input:&Vec<char>) -> bool {
        let pos = Position { position: None, length: input.len() };
        let result = self.matches_index(input, 0, pos);
        match result {
            Some(next) => match self.matches(input, &RuleImplementation::End, next) {
                Some(_) => true,
                None => false
            },
            None => false
        }
    }

    fn matches_index(&self, input:&Vec<char>, index:usize, pos:Position) -> Option<Position> {
        let rule = &self.rules[&index];
        self.matches(input, &rule.implementation, pos)
    }

    fn matches(&self, input:&Vec<char>, rule:&RuleImplementation, pos:Position) -> Option<Position> {
        match rule {
            RuleImplementation::Character(match_c) => match pos.next() {
                None => None,
                Some((c, next)) => if input[c] == *match_c { Some(next) } else { None }
            },
            RuleImplementation::Rule(index) => self.matches_index(input, *index, pos),
            RuleImplementation::Any(rules) => {
                for rule in rules {
                    if let Some(next) = self.matches(input, rule, pos) {
                        return Some(next);
                    }
                }
                None
            },
            RuleImplementation::Each(rules) => {
                let mut next = pos;
                for sub_rule in rules {
                    next = match self.matches(input, sub_rule, next) {
                        None => { return None; },
                        Some(n) => n
                    }
                }
                Some(next)
            },
            RuleImplementation::End => {
                match pos.next() {
                    None => Some(pos),
                    Some(_) => None
                }
            }
            _ => unimplemented!()
        }
    }
}

#[derive(Clone,Copy)]
struct Position {
    length: usize,
    position: Option<usize>
}

impl Position {
    fn next(&self) -> Option<(usize, Position)> {
        let next_pos = match self.position {
            None => 0,
            Some(pos) => pos + 1
        };
        if next_pos >= self.length {
            None
        } else {
            Some((next_pos, Position { length: self.length, position: Some(next_pos) }))
        }
    }
}
