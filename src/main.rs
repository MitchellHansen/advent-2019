extern crate reqwest;
extern crate tempfile;

mod problem1;
mod problem2;
mod util;

fn main() {

    let problem1_data = util::get_problem(1);

    problem1::part1::run(&problem1_data);
    problem1::part2::run(&problem1_data);

}
