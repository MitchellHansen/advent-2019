use crate::Problem;
use std::str::FromStr;
use std::collections::HashSet;
use std::thread::yield_now;

pub struct Problem4 {
    input: Vec<String>,
}

impl Problem4 {
}

impl Problem for Problem4 {
    fn new(input: &String) -> Self {
        Problem4 {
            input: input.split("-").map(|s| String::from(s.to_string().trim())).collect()
        }
    }


    fn run_part1(&self) {
        let mut count = 0;
        let start = self.input[0].parse::<i32>().unwrap();
        let end = self.input[1].parse::<i32>().unwrap();

        for i in start..end {
            let v = i.to_string();

            let mut crit_3 = false;
            let mut prev = '0';
            let mut count_vec = Vec::new();
            count_vec.resize(10 as usize, 0);

            for i in v.chars() {
                if i == prev {
                    crit_3 = true;
                    if count_vec[i.to_digit(10).unwrap() as usize] == 0 {
                        count_vec[i.to_digit(10).unwrap() as usize] += 2
                    } else {
                        count_vec[i.to_digit(10).unwrap() as usize] += 1
                    }

                }
                prev = i;
            }

            let mut crit_5 = false;
            for i in count_vec {
                if i == 2 {
                    crit_5 = true;
                }
            }

            if crit_3 == false || crit_5 == false {
                continue
            }

            let mut crit_4 = true;
            let mut prev = 0;
            for i in v.chars() {
                if i.to_digit(10).unwrap() < prev {
                    crit_4 = false;
                    break;
                }
                prev = i.to_digit(10).unwrap();
            }

            if crit_3 && crit_4 {
                count += 1;
            }
        }

        println!("{:?}", count);
    }

    fn run_part2(&self) {
    }
}

