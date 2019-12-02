extern crate reqwest;
extern crate tempfile;

mod problem1;
mod problem2;
mod util;

// struct to represent commands, opcodes
// 1 add togerther two numbers and stores a third
// 2 multiplies using the same deal (op, in, in, out)



fn main() {

  //  let problem1_data = util::get_problem(1);

    //problem1::part1::run(&problem1_data);
    //problem1::part2::run(&problem1_data);

    let problem2_data = util::get_problem(2);

    problem2::part2::run(&problem2_data);

}
