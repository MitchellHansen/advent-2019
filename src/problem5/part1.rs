use crate::Problem;
use std::io;

#[derive(PartialEq)]
enum OpCode {
    ADD = 1,
    MULT = 2,
    INPUT = 3,
    OUTPUT = 4,
    JMP_T = 5,
    JMP_F = 6,
    LESS = 7,
    EQUAL = 8,
    HALT = 99,
}

enum Mode {
    ADDRESSING = 1,
    ACTUAL = 2,
}

pub struct Problem5 {
    input: Vec<i32>,
}

impl Problem5 {

    fn opcode(input: i32) -> Option<OpCode>{
        match input {
            1 => {
                Some(OpCode::ADD)
            },
            2 => {
                Some(OpCode::MULT)
            },
            3 => {
                Some(OpCode::INPUT)
            },
            4 => {
                Some(OpCode::OUTPUT)
            },
            5 => {
                Some(OpCode::JMP_T)
            },
            6 => {
                Some(OpCode::JMP_F)
            },
            7 => {
                Some(OpCode::LESS)
            },
            8 => {
                Some(OpCode::EQUAL)
            },
            99 => {
                Some(OpCode::HALT)
            }
            _ => None
        }
    }

    fn mode(input: &String, index: i32) -> Mode {

        if input.len() < index as usize {
            Mode::ADDRESSING
        } else {
            match input.chars().rev().nth(index as usize).unwrap() {
                '0' => {
                    Mode::ADDRESSING
                },
                '1' => {
                    Mode::ACTUAL
                },
                _ => {
                    panic!("Bad mode");
                }
            }
        }
    }

    fn get(id: i32, memory: &Vec<i32>, mode: Mode) -> i32 {
        match mode {
            Mode::ADDRESSING => {
                *memory.get(id as usize).unwrap()
            },
            Mode::ACTUAL => {
                id
            }
        }
    }
}

impl Problem for Problem5 {
    fn new(input: &String) -> Self {
        Problem5 {
            input: input
                .split(",")
                .filter_map(|s| {
                    let s = s.trim();
                    if !s.is_empty() {
                        Some(s.parse::<i32>().unwrap())
                    } else {
                        None
                    }
                }).collect(),
        }
    }

    fn run_part1(&self) {

        let mut memory = self.input.clone();

        memory = self.input.clone();

        let mut id = 0;

        while true {

            let intcode = format!("0000000000{}", memory.get(id as usize).unwrap().to_string());

            let modes = &intcode[..intcode.len()-2].to_string();
            let opcode = &intcode[intcode.len()-2..].parse().unwrap();

            let opcode = match Problem5::opcode(*opcode) {
                Some(code) => code,
                None => OpCode::HALT
            };

            match opcode {
                OpCode::ADD => {

                    let in1 = *memory.get((id + 1) as usize).unwrap();
                    let in2 = *memory.get((id + 2) as usize).unwrap();

                    let val1 = Problem5::get(in1, &memory, Problem5::mode(modes, 0));
                    let val2 = Problem5::get(in2, &memory, Problem5::mode(modes, 1));

                    let out = memory.get((id + 3) as usize).unwrap().clone();

                    memory[((out) as usize)] = (val1 + val2);

                    id += 4;
                },
                OpCode::MULT => {

                    let in1 = *memory.get((id + 1) as usize).unwrap();
                    let in2 = *memory.get((id + 2) as usize).unwrap();

                    let val1 = Problem5::get(in1, &memory, Problem5::mode(modes, 0));
                    let val2 = Problem5::get(in2, &memory, Problem5::mode(modes, 1));

                    let out = memory.get((id + 3) as usize).unwrap().clone();

                    memory[((out) as usize)] = (val1 * val2);

                    id += 4;
                },
                OpCode::JMP_T => {

                    let in1 = *memory.get((id + 1) as usize).unwrap();
                    let in2 = *memory.get((id + 2) as usize).unwrap();

                    let val1 = Problem5::get(in1, &memory, Problem5::mode(modes, 0));
                    let val2 = Problem5::get(in2, &memory, Problem5::mode(modes, 1));

                    if val1 != 0 {
                        id = val2;
                    } else {
                        id += 3;
                    }
                },
                OpCode::JMP_F => {

                    let in1 = *memory.get((id + 1) as usize).unwrap();
                    let in2 = *memory.get((id + 2) as usize).unwrap();

                    let val1 = Problem5::get(in1, &memory, Problem5::mode(modes, 0));
                    let val2 = Problem5::get(in2, &memory, Problem5::mode(modes, 1));

                    if val1 == 0 {
                        id = val2;
                    } else {
                        id += 3;
                    }
                },
                OpCode::LESS => {

                    let in1 = *memory.get((id + 1) as usize).unwrap();
                    let in2 = *memory.get((id + 2) as usize).unwrap();

                    let val1 = Problem5::get(in1, &memory, Problem5::mode(modes, 0));
                    let val2 = Problem5::get(in2, &memory, Problem5::mode(modes, 1));

                    let out = memory.get((id + 3) as usize).unwrap().clone();
                    if val1 < val2 {
                        memory[((out) as usize)] = 1;
                    } else {
                        memory[((out) as usize)] = 0;
                    }
                    id += 4;

                },
                OpCode::EQUAL => {

                    let in1 = *memory.get((id + 1) as usize).unwrap();
                    let in2 = *memory.get((id + 2) as usize).unwrap();

                    let val1 = Problem5::get(in1, &memory, Problem5::mode(modes, 0));
                    let val2 = Problem5::get(in2, &memory, Problem5::mode(modes, 1));

                    let out = memory.get((id + 3) as usize).unwrap().clone();
                    if val1 == val2 {
                        memory[((out) as usize)] = 1;
                    } else {
                        memory[((out) as usize)] = 0;
                    }
                    id += 4;
                },
                OpCode::INPUT => {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)
                        .expect("Couldn't read input");
                    let input: i32 = input.trim().parse()
                        .expect("Couldn't parse input");
                    let i = *memory.get((id + 1) as usize).unwrap();
                    memory[i as usize] = input;
                    id += 2;
                },
                OpCode::OUTPUT => {
                    let i = *memory.get((id + 1) as usize).unwrap();
                    let v = Problem5::get(i, &memory, Problem5::mode(modes, 0));
                    println!("{:?}", v);
                    id += 2;
                },
                OpCode::HALT => {
                    break
                }
            }
        }

        // -12628
        println!("{:?}", memory);
    }

    fn run_part2(&self) {
            }
}

