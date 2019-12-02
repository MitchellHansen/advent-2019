/*
Fuel required to launch a given module is based on its mass. Specifically, to find the fuel
required for a module, take its mass, divide by three, round down, and subtract 2.

fuel = floor(mass / 3) - 2
*/

use crate::Problem;

pub struct Problem1 {
    mass_list: Vec<i32>,
}

impl Problem1 {
    fn rocket_equation(mass: i32) -> i32 {

        let mut mass_left = mass;
        let mut fuel_load = 0;

        while ((mass_left as f32 / 3.0).floor() as i32 - 2) > 0 {
            let fuel_needed = (mass_left as f32 / 3.0).floor() as i32 - 2;
            mass_left = fuel_needed;
            fuel_load = fuel_load + fuel_needed;
        }

        fuel_load
    }
}

impl Problem for Problem1 {

    fn new(input: &String) -> Self {
        Problem1 {
            mass_list: input
                .split("\n")
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
        let mut fuel = 0;

        for v in self.mass_list.iter() {
            fuel = fuel + (*v as f32 / 3.0).floor() as i32 - 2;
        }

        println!("Fuel is = {}", fuel);
    }

    fn run_part2(&self) {
        let mut fuel = 0;

        for v in self.mass_list.iter() {
            fuel = fuel + Problem1::rocket_equation(*v);
        }

        println!("Rocket equation fuel is = {}", fuel);
    }
}

