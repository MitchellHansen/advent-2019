extern crate reqwest;
extern crate tempfile;

use crate::problem1::part1::Problem1;
use crate::problem2::part1::Problem2;
use crate::problem3::part1::Problem3;
use crate::problem4::part1::Problem4;
use crate::problem5::part1::Problem5;
use crate::problem6::part1::Problem6;

mod problem1;
mod problem2;
mod problem3;
mod problem4;
mod problem5;
mod problem6;
mod util;

pub trait Problem {
    // Parses input and generates state
    fn new(input: &String) -> Self;

    // Runs on state
    fn run_part1(&self);
    fn run_part2(&self);
}

fn main() {

//    let problem1 = Problem1::new(&util::get_problem(1));
//    problem1.run_part1();
//    problem1.run_part2();
//
//    let problem2 = Problem2::new(&util::get_problem(2));
//    problem2.run_part1();
//    problem2.run_part2();
//
//    let problem3 = Problem3::new(&util::get_problem(3));
//    problem3.run_part1();
//    problem3.run_part2();
//
//    let problem4 = Problem4::new(&util::get_problem(4));
//    problem4.run_part1();
//    problem4.run_part2();
//
//    let problem5 = Problem5::new(&util::get_problem(5));
//    problem5.run_part1();
//
    let problem6 = Problem6::new(&util::get_problem(6));
    //problem6.run_part1();
    problem6.run_part2();

}
