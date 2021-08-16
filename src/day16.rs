

use std::collections::HashMap;
use core::str::FromStr;
use core::convert::Infallible;
use std::collections::HashSet;

pub fn execute() {
    /* Part 1
    let file = std::fs::read_to_string(".\\data\\16.txt").unwrap();

    let sections:Vec<&str> = file.split("\n\n").collect();
    let rules:Vec<Rule> = sections[0].split("\n").map(|line| line.parse().unwrap()).collect();
    let tickets:Vec<Vec<i32>> = sections[2].split("\n").skip(1)
        .filter(|line| line.len() > 0)
        .map(|line| ).collect();
    
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
    */

    let file = std::fs::read_to_string(".\\data\\16.txt").unwrap();

    let sections:Vec<&str> = file.split("\n\n").collect();
    let mut fields:FieldList = FieldList::new(sections[0].split("\n").map(|line| line.parse().unwrap()).collect());
    let my_ticket:Ticket = sections[1].split("\n").last().unwrap().parse().unwrap();
    let my_ticket_clone:Ticket = sections[1].split("\n").last().unwrap().parse().unwrap();
    let mut tickets:TicketList = TicketList::new(
        sections[2].split("\n").skip(1)
            .filter(|line| line.len() > 0)
            .map(|line| line.parse().unwrap()).collect()
    );
    tickets.add_ticket(my_ticket_clone);
    
    println!("Total tickets: {}", tickets.len());

    tickets.remove_invalid(&fields);

    println!("Valid tickets: {}", tickets.len());

    fields.fill_possible_columns(&tickets);
    fields.iterate_possibilities();
    let departure_columns = fields.departure_field_columns();

    let total:i64 = departure_columns.iter().fold(1, |acc, &col| {
        println!("{} {}", col, my_ticket.values[col]);
        acc * (my_ticket.values[col] as i64)
    });

    println!("{}", total);
}

struct TicketList {
    tickets:Vec<Ticket>
}

impl TicketList {
    pub fn new(tickets:Vec<Ticket>) -> TicketList{
        TicketList {
            tickets:tickets
        }
    }

    pub fn add_ticket(&mut self, ticket:Ticket) {
        self.tickets.push(ticket)
    }

    pub fn remove_invalid(&mut self, fields:&FieldList) {
        self.tickets.retain(|ticket| {
            !ticket.not_valid_for_any_rule(fields)
        })
    }

    pub fn len(&self) -> usize {
        self.tickets.len()
    }
}

struct Ticket {
    values:Vec<i32>
}

impl Ticket {
    pub fn not_valid_for_any_rule(&self, fields:&FieldList) -> bool {
        self.values.iter()
            .any(|num| { 
                !fields.any_field_matches(*num)
            })
    }
}

impl FromStr for Ticket {
    type Err = Infallible;

    
    fn from_str(line: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> { 
        let vec:Vec<i32> = line.split(',').map(|num| num.parse().unwrap()).collect();
        Ok(Ticket {
            values:vec
        })
    }
}

struct Field {
    field_name: String,
    first_lower: i32,
    first_upper: i32,
    second_lower: i32,
    second_upper: i32,
    possible_columns: Vec<usize>,
    solved_column: Option<usize>
}

impl Field {
    fn is_valid_for(&self, number:i32) -> bool {
        (number >= self.first_lower && number <= self.first_upper)
        || (number >= self.second_lower && number <= self.second_upper)
    }

    fn fill_possible_columns(&mut self, tickets:&TicketList, column_count:usize) {
        let viable_columns:Vec<usize> = (0..column_count).filter(|index| {
            let is_not_valid = tickets.tickets.iter().any(|ticket| {
                !self.is_valid_for(ticket.values[*index])
            });
            !is_not_valid
        }).collect();
        self.possible_columns = viable_columns;
    }
}

impl FromStr for Field {
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

        Ok(Field {
            field_name: name.to_owned(),
            first_lower: first_lower,
            first_upper: first_upper,
            second_lower: second_lower,
            second_upper: second_upper,
            possible_columns: Vec::new(),
            solved_column: None
        })
     }
}

struct FieldList {
    rules:Vec<Field>
}

impl FieldList {
    pub fn new(rules:Vec<Field>) -> FieldList{
        FieldList {
            rules: rules
        }
    }

    pub fn any_field_matches(&self, number:i32) -> bool {
        self.rules.iter().any(|rule| { rule.is_valid_for(number) })
    }

    fn fill_possible_columns(&mut self, tickets:&TicketList) {
        let count = self.rules.len();

        for field in self.rules.iter_mut() {
            field.fill_possible_columns(tickets, count);
        }
    }

    fn print_possibilities(&self) {
        for field in self.rules.iter() {
            let possible_count = field.possible_columns.len();
            println!("{} - {}", field.field_name, possible_count);
        }
    }

    fn iterate_possibilities(&mut self) {
        let mut any_multiples = self.rules.iter().any(|field| {  field.possible_columns.len() > 0 });
        let mut max_attempts = 1000;
        while any_multiples {
            let single_column_index = self.rules
                .iter()
                .enumerate()
                .filter(|(_, field)| {  field.possible_columns.len() == 1 })
                .map(|(index, _)| { index  })
                .next()
                .unwrap();

            let solved_column = self.rules[single_column_index].possible_columns[0];
            self.rules[single_column_index].solved_column = Some(solved_column);

            for rule in self.rules.iter_mut() {
                rule.possible_columns.retain(|x| {
                    *x != solved_column
                });
            }

            any_multiples = self.rules.iter().any(|field| {  field.possible_columns.len() > 0 });
            max_attempts -= 1;
            if max_attempts <= 0 {
                panic!("Infinite loop!");
            }
        }
    }

    fn departure_field_columns(&self) -> Vec<usize> {
        self.rules.iter().filter(|field| { field.field_name.starts_with("departure ") })
            .map(|field| { field.solved_column.unwrap() }).collect()
    }
}