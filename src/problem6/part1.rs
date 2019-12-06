use crate::Problem;
use std::io;
use std::collections::{HashSet, HashMap};
use std::hash::Hash;
use std::collections::hash_map::Entry;

struct Body<'a> {
    id: String,
    parent: &'a Body<'a>,
}

pub struct Problem6 {
    input: Vec<(String, String)>,
}

impl Problem6 {
}

impl Problem for Problem6 {
    fn new(input: &String) -> Self {
        Problem6 {
            input: input
                .split("\n")
                .filter_map(|s| { // AAA)BBB
                    let s = s.trim();
                    if !s.is_empty() {
                        let mut split = s.split(")");
                        let parent = split.next().unwrap().to_string();
                        let child = split.next().unwrap().to_string();
                        Some((parent, child))
                    } else {
                        None
                    }
                }).collect(),
        }
    }

    fn run_part1(&self) {

        //                       child   parent
        let mut bodies : HashMap<String, String> = HashMap::with_capacity(self.input.len());

        for i in &self.input {
            bodies.insert(i.1.clone(), i.0.clone());
        }

        let mut orbit_count = 0;

        for i in &self.input {

            let mut current = i.1.clone();

            while let Some(v) = bodies.get(current.as_str()){

                orbit_count += 1;

                if v == "COM" {
                    break;
                } else {
                    current = v.clone();
                }
            }
        }

        print!("{:?}", orbit_count);
    }

    fn run_part2(&self) {
        //                       child   parent
        let mut bodies : HashMap<String, String> = HashMap::with_capacity(self.input.len());

        for i in &self.input {
            bodies.insert(i.1.clone(), i.0.clone());
        }


        let mut my_path = Vec::new();
        let mut santas_path = Vec::new();


        let mut current = String::from("YOU");

        while let Some(v) = bodies.get(current.as_str()){

            my_path.push(v.clone());

            if v == "COM" {
                break;
            } else {
                current = v.clone();
            }
        }

        let mut current = String::from("SAN");

        while let Some(v) = bodies.get(current.as_str()){

            santas_path.push(v.clone());

            if v == "COM" {
                break;
            } else {
                current = v.clone();
            }
        }


        for i in &my_path {
            let mut count = 0;
            for v in &santas_path {
                count += 1;
                if v == i {
                    println!("{}, {}", i, count);
                    break;
                }
            }
        }

        println!("{:?}", my_path);


        for i in &santas_path {
            let mut count = 0;
            for v in &my_path {
                count += 1;
                if v == i {
                    println!("{}, {}", i, count);
                    break;
                }
            }
        }

        println!("{:?}", santas_path);
    }
}

