use crate::Problem;
use std::str::FromStr;
use std::collections::HashSet;
use std::thread::yield_now;

pub struct Problem3 {
    input: Vec<Vec<Operation>>,
}

impl Problem3 {

}

impl Problem for Problem3 {
    fn new(input: &String) -> Self {
        Problem3 {
            input: input
                .split("\n").filter_map(|list| {
                    let list = list.trim();
                    if !list.is_empty() {
                        Some(list.split(',').filter_map(|s| {
                            let s = s.trim();
                            if !s.is_empty() {
                                Some(Operation::from_str(s).unwrap())
                            } else {
                                None
                            }
                        }).collect())
                    } else {
                        None
                    }

                }).collect(),
        }
    }

    fn run_part1(&self) {
    }

    fn run_part2(&self) {
    }
}

