
use std::collections::HashSet;
use core::str::FromStr;
use core::convert::Infallible;
use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

pub fn execute() {
    let file = File::open(".\\data\\08.txt").unwrap();
    let reader = BufReader::new(&file);
    let instructions:Vec<Instruction> = reader.lines().map(|line| { line.unwrap().parse::<Instruction>().unwrap() }).collect();

    let mut vm = VirtualMachine::new(&instructions);

    let (acc, found_infinite_loop) = vm.execute();
    println!("{}, {:?}", acc, found_infinite_loop);

    let possible_flippers = vm.find_possible_flipped();
    println!("{}", possible_flippers.len());

    for flipper in possible_flippers {
        let mut vm_with_flip = VirtualMachine::new_with_flipped(&instructions, flipper.clone());
        let (acc, found_infinite_loop) = vm_with_flip.execute();
        if found_infinite_loop != ExecutionState::EndInfiniteLoop {
            println!("{} {} {:?}", flipper, acc, found_infinite_loop);
        }
    }
    
}

fn move_instruction_pointer(pointer:usize, delta:i32) -> Option<usize> {
    if delta < 0 {
        pointer.checked_sub(delta.abs() as usize)
    } else {
        pointer.checked_add(delta as usize)
    }
}

#[derive(PartialEq, Debug)]
enum ExecutionState {
    Executing,
    EndInfiniteLoop,
    EndInvalidProgram,
    End
}

struct VirtualMachine<'a> {
    instructions:&'a Vec<Instruction>,
    executed_instructions:HashSet<usize>,
    instruction_pointer:usize,
    flipped_instruction: Option<usize>
}

impl VirtualMachine<'_> {
    fn new(instructions:&Vec<Instruction>) -> VirtualMachine {
        VirtualMachine {
            instructions: instructions,
            executed_instructions: HashSet::new(),
            instruction_pointer: 0,
            flipped_instruction: None
        }
    }

    fn new_with_flipped(instructions:&Vec<Instruction>, flipped_instruction: usize) -> VirtualMachine {
        VirtualMachine {
            instructions: instructions,
            executed_instructions: HashSet::new(),
            instruction_pointer: 0,
            flipped_instruction: Some(flipped_instruction)
        }
    }

    fn find_possible_flipped(&self) -> Vec<usize> {
        self.instructions.iter().enumerate().filter(|(index, instruction)| {
            let move_to = match &instruction.command.as_str() {
                &"nop" => move_instruction_pointer(index.clone(), instruction.value),
                &"jmp" => move_instruction_pointer(index.clone(), 1),
                _ => None
            };
            match move_to {
                None => false,
                Some(pointer) => pointer < self.instructions.len()// && vm.executed_instructions.contains(&pointer)
            }
        }).map(|(index, _)| { index})
        .collect()
    }

    fn get_current_instruction(&self) -> Option<Instruction> {
        let instruction_attempt = self.instructions.get(self.instruction_pointer);
        match instruction_attempt {
            Some(instruction) => {
                match self.flipped_instruction {
                    Some(inst) if inst == self.instruction_pointer => {
                        match &instruction.command.as_str() {
                            &"jmp" => Some(Instruction{ command: "nop".to_owned(), value: 0 }),
                            &"nop" => Some(Instruction{ command: "jmp".to_owned(), value: instruction.value.clone() }),
                            _ => Some(instruction.clone())
                        }
                    },
                    _ => Some(instruction.clone())
                }
            }
            None => None
        }
    }

    fn execute(&mut self) -> (i32, ExecutionState) {
        let mut state:ExecutionState = ExecutionState::Executing;
        let mut accumulator:i32 = 0;

        while state == ExecutionState::Executing {
            state = if self.instruction_pointer == self.instructions.len() {
                ExecutionState::End
            } else if self.executed_instructions.contains(&self.instruction_pointer) {
                ExecutionState::EndInfiniteLoop
            } else {
                self.executed_instructions.insert(self.instruction_pointer);
                let instruction_attempt = self.get_current_instruction();
                match instruction_attempt {
                    Some(instruction) => {
                        self.instruction_pointer = match &instruction.command.as_str() {
                            &"acc" => { accumulator += instruction.value; self.instruction_pointer + 1 }
                            &"nop" => { self.instruction_pointer + 1 },
                            &"jmp" => { move_instruction_pointer(self.instruction_pointer, instruction.value).unwrap() }
                            _ => todo!()
                        };
                        ExecutionState::Executing
                    },
                    None => ExecutionState::EndInvalidProgram
                }
                
            }
        }

        (accumulator, state)
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    command: String,
    value: i32
}

impl FromStr for Instruction {
    type Err = Infallible;

    fn from_str(line: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> { 
        let name = line[0..3].to_owned();
        let value:i32 = line[4..].parse().unwrap();
        Ok(Instruction {
            command: name,
            value: value
        })
     }
}