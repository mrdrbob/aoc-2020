
use std::collections::VecDeque;
use std::collections::HashMap;
use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

#[derive(Debug)]
enum Instruction {
    // Bitmask(u64, u64), // (set mask, unset mask)
    Bitmask(u64, u64, Vec<u64>), // set mask, float mask, float variants
    MemSet(u64, u64) // mem address, value
}

pub fn execute() {
    let file = File::open(".\\data\\14.txt").unwrap();
    let reader = BufReader::new(&file);
    let mut memory:HashMap<u64, u64> = HashMap::new();


    let instructions:Vec<Instruction> = reader.lines().map(|line| {
        let l = line.unwrap();
        match &l[..3] {
            "mas" => {
                let set_mask = u64::from_str_radix(l.chars().skip(7).map(|x| { if x == '1' { '1' } else { '0' } }).collect::<String>().as_str(), 2).unwrap();
                let flip_mask = u64::from_str_radix(l.chars().skip(7).map(|x| { if x == 'X' { '1' } else { '0' } }).collect::<String>().as_str(), 2).unwrap();
                let flip_variants = generate_flip_variations(flip_mask);


                Instruction::Bitmask(set_mask, flip_mask, flip_variants)
            },
            "mem" => {
                let split:Vec<&str> = l[4..].split("] = ").collect();
                let address:u64 = split.get(0).unwrap().parse().unwrap();
                let value:u64 = split.get(1).unwrap().parse().unwrap();
                Instruction::MemSet(address, value)
            },
            _ => panic!()
        }
    }).collect();

    let mut set:u64 = 0;
    let mut flip_mask:u64 = 0;
    let mut flip_bits:Vec<u64> = Vec::new();

    for instruction in instructions {
        match instruction {
            Instruction::Bitmask(s, mask, flips) => { set = s; flip_mask = mask; flip_bits = flips; },
            Instruction::MemSet(address, value) => {
                let updated_address = address | set;

                for i in 0..flip_bits.len() {
                    let flip = flip_bits.get(i).unwrap();
                    let zeroed = updated_address & !flip_mask;
                    let filled = zeroed | flip;
                    // println!("Mem: {}, Val: {}", filled, value);

                    memory.remove(&filled);
                    memory.insert(filled, value);
                }
            }
        }
    }

    /*
    Part 1
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
    */

    let sum:u64 = memory.values().sum();
    println!("{}", sum);
}

fn find_next_one(number:u64, index:i32) -> Option<i32> {
    let mut index_to_test = index  -1;
    while index_to_test >= 0 && (number & (1 << index_to_test)) == 0 {
        index_to_test -= 1;
    }
    if index_to_test < 0 {
        None
    } else {
        Some(index_to_test)
    }
}

fn generate_flip_variations(number:u64) -> Vec<u64> {
    let mut output:Vec<u64> = Vec::new();

    let mut stack:VecDeque<(i32, u64)> = VecDeque::new(); // index, numbers to expand.

    // I checked the input, and all masks have at least 1 X, so I'm safe to find the first (from the left, thus most significant)
    // floating bit and push it on to the stack.
    stack.push_back((find_next_one(number, 37).unwrap(), 0));

    while stack.len() > 0 {
        let (index, variant) = stack.pop_back().unwrap();

        // First, I create both variations for this bit.
        // The set variation, I set the bit to 1.
        let set_variant = variant | (1 << index);
        // For the unset variation, I do nothing, because I started with 0, it's already zero for that bit.
        let clear_variant = variant;

        // Now I look for the index of the next (less significant) floating bit.
        let next_index = find_next_one(number, index);
        match next_index {
            None => {
                // If no next floating bit is found, then there are no more possible variations for these
                // two variants, so I put them in the output list.
                output.push(set_variant);
                output.push(clear_variant);
            },
            Some(i) => {
                // If there is another index, then each of these variants will also have variants, so I
                // push these two on the stack along with the next index.
                stack.push_back((i, set_variant));
                stack.push_back((i, clear_variant));
            }
        }
    }

    // Once the stack is exhausted, then all possible combinations have been created.
    output
}