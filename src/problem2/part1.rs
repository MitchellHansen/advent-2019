

#[derive(PartialEq)]
enum OpCode {
    ADD = 1,
    MULT = 2,
    HALT = 99,
}

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
pub fn run(input: &Vec<i32>) {

    let mut memory = input.clone();
    memory[(1 as usize)] += 12;
    memory[(2 as usize)] += 2;

    println!("{:?}", memory);

    let mut id = 0;

    while opcode(id, &memory).unwrap() != OpCode::HALT {

        let opcode =  opcode(id, &memory).unwrap();

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
