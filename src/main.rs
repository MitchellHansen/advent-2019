extern crate reqwest;
extern crate tempfile;

use crate::problem1::part1::Problem1;
use crate::problem2::part1::Problem2;

mod problem1;
mod problem2;
mod util;

pub trait Problem {
    // Parses input and generates state
    fn new(input: &String) -> Self;

    // Runs on state
    fn run_part1(&self);
    fn run_part2(&self);
}

fn main() {

    let problem1 = Problem1::new(&util::get_problem(1));

    problem1.run_part1();
    problem1.run_part2();

    let problem2 = Problem2::new(&util::get_problem(2));

    problem2.run_part1();
    problem2.run_part2();

}
