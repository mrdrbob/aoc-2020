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
        // let is_match = rules.check_match(&t);
        let is_match = run_part_2(&rules, &t);
        if is_match {
            println!("{}", line);
        }
        if is_match { 1 } else { 0 }
    }).sum();
    println!("{}", match_count);
}

fn run_part_2(rules:&RuleList, line:&Vec<char>) -> bool {
    // We don't run rule 0. Instead we run a.. version of it.
    // Rule 0 is defined as: `0: 8 11`

    let pos = Position { position: None, length: line.len() };

    // Rule 8 is `8: 42 | 42 8`... effectively, run rule `42` at least once, as many times as possible. Easy:
    let answer_to_life_universe_and_everything = &rules.rules[&42];
    let (next, count_42) = rules.matches_many_times(line, &answer_to_life_universe_and_everything.implementation, pos);

    // The next rule ALSO requires 42, but because `matches_many_times` is greedy, it's already consumed all the input that 
    // matches 42... However, if it's matched at least twice, then we've satisfied the "at least once" part of both rule 8 and 11.
    // So if we didn't get at least 2 42 matches, it's invalid.
    if count_42 < 2 {
        return false;
    }

    // Rule 11 is `11: 42 31 | 42 11 31`, so it's 42 at least once, followed by 31 possibly multiple times (depending on how many times 
    // the second half is matched). Again, rule 8 has already consumed all the 42 matching input, so long as it matched at least twice, 
    // we can ignore the "42 at least once" rule. Instead we just match 31 repeatedly.
    let rule_31 = &rules.rules[&31];
    let (next, count_31) = rules.matches_many_times(line, &rule_31.implementation, next);

    let end_of_input = match next.next() {
        None => true,
        Some(_) => false
    };

    // So finally, for this all to be correct, we need to have matched 31 at least once,
    // 42 matched more than 31, and we're out of input
    count_31 > 0 && count_42 > count_31 && end_of_input
}

#[derive(Debug)]
struct Rule {
    id: usize,
    implementation: RuleImplementation
}

#[derive(Debug)]
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

    fn matches_many_times(&self, input:&Vec<char>, rule:&RuleImplementation, pos:Position) -> (Position, usize) {
        let mut times_run:usize = 0;
        let mut next = pos;

        while let Some(n) = self.matches(input, rule, next) {
            next = n;
            times_run += 1;
        }

        (next, times_run)
    }

    fn matches(&self, input:&Vec<char>, rule:&RuleImplementation, pos:Position) -> Option<Position> {
        // println!("{:?}", rule);
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

#[derive(Clone,Copy,Debug)]
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
