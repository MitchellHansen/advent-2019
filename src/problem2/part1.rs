use crate::Problem;

#[derive(PartialEq)]
enum OpCode {
    ADD = 1,
    MULT = 2,
    HALT = 99,
}

pub struct Problem2 {
    input: Vec<i32>,
}

impl Problem2 {
    fn opcode(input: i32, memory: &Vec<i32>) -> Option<OpCode>{
        match memory.get(input as usize).unwrap() {
            1 => {
                Some(OpCode::ADD)
            },
            2 => {
                Some(OpCode::MULT)
            },
            99 => {
                Some(OpCode::HALT)
            }
            _ => None
        }
    }
}

impl Problem for Problem2 {
    fn new(input: &String) -> Self {
        Problem2 {
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
        memory[(1 as usize)] += 12;
        memory[(2 as usize)] += 2;

        println!("{:?}", memory);

        let mut id = 0;

        while Problem2::opcode(id, &memory).unwrap() != OpCode::HALT {

            let opcode =  Problem2::opcode(id, &memory).unwrap();

            let in1 = memory.get((id + 1) as usize).unwrap();
            let in2 = memory.get((id + 2) as usize).unwrap();

            let val1 = memory.get((*in1) as usize).unwrap();
            let val2 = memory.get((*in2) as usize).unwrap();

            let out = memory.get((id + 3) as usize).unwrap().clone();

            match opcode {
                OpCode::ADD => {
                    memory[((out) as usize)] = (val1 + val2);
                },
                OpCode::MULT => {
                    memory[((out) as usize)] = (val1 * val2);
                },
                _ => {}
            }

            id += 4;
        }


        println!("{:?}", memory);
    }

    fn run_part2(&self) {
        let mut memory = self.input.clone();

        let mut x = 0;
        let mut y = 0;

        while memory[(0 as usize)] != 19690720 {

            memory = self.input.clone();

            if x > 99 {
                x = 0;
                y += 1;
            }
            if y > 99 {
                panic!("This should't happen");
            }

            memory[(1 as usize)] = x;
            memory[(2 as usize)] = y;

            let mut id = 0;

            while true {

                let opcode = match Problem2::opcode(id, &memory) {
                    Some(code) => code,
                    None => OpCode::HALT
                };

                let in1 = memory.get((id + 1) as usize).unwrap();
                let in2 = memory.get((id + 2) as usize).unwrap();

                let val1 = memory.get((*in1) as usize).unwrap();
                let val2 = memory.get((*in2) as usize).unwrap();

                let out = memory.get((id + 3) as usize).unwrap().clone();

                match opcode {
                    OpCode::ADD => {
                        memory[((out) as usize)] = (val1 + val2);
                    },
                    OpCode::MULT => {
                        memory[((out) as usize)] = (val1 * val2);
                    },
                    OpCode::HALT => {
                        break
                    }
                }

                id += 4;
            }
            x += 1;
        }

        println!("{:?}", memory);
    }
}

