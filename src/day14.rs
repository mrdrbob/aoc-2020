
use std::collections::HashMap;
use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

#[derive(Debug)]
enum Instruction {
    Bitmask(u64, u64), // (set mask, unset mask)
    MemSet(usize, u64) // mem address, value
}

pub fn execute() {
    let file = File::open(".\\data\\14.txt").unwrap();
    let reader = BufReader::new(&file);
    let mut memory:HashMap<usize, u64> = HashMap::new();

    let instructions:Vec<Instruction> = reader.lines().map(|line| {
        let l = line.unwrap();
        match &l[..3] {
            "mas" => {
                let set_mask = u64::from_str_radix(l.chars().skip(7).map(|x| { if x == '1' { '1' } else { '0' } }).collect::<String>().as_str(), 2).unwrap();
                let unset_mask = u64::from_str_radix(l.chars().skip(7).map(|x| { if x == '0' { '0' } else { '1' } }).collect::<String>().as_str(), 2).unwrap();
                Instruction::Bitmask(set_mask, unset_mask)
            },
            "mem" => {
                let split:Vec<&str> = l[4..].split("] = ").collect();
                let address:usize = split.get(0).unwrap().parse().unwrap();
                let value:u64 = split.get(1).unwrap().parse().unwrap();
                Instruction::MemSet(address, value)
            },
            _ => panic!()
        }
    }).collect();

    println!("{:?}", instructions.get(0).unwrap());
    println!("{:?}", instructions.get(1).unwrap());

    let mut set = 0;
    let mut unset = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Bitmask(s, u) => { set = s; unset = u; },
            Instruction::MemSet(address, value) => {
                let updated_value = value | set;
                let updated_value2 = updated_value & unset;
                memory.remove(&address);
                memory.insert(address, updated_value2);
            }
        }
    }

    let sum:u64 = memory.values().sum();
    println!("{}", sum);
}
