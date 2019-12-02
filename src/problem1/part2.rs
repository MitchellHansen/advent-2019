


/*

Fuel required to launch a given module is based on its mass. Specifically, to find the fuel
required for a module, take its mass, divide by three, round down, and subtract 2.

fuel = floor(mass / 3) - 2


*/


extern crate reqwest;
extern crate tempfile;

use std::io::copy;
use std::fs::File;
use tempfile::Builder;
use futures::executor::block_on;
use std::collections::HashMap;
use reqwest::cookie::Cookie;
use reqwest::header::HeaderValue;

pub fn rocket_equation(mass: i32) -> i32 {

    let mut mass_left = mass;
    let mut fuel_load = 0;

    while ((mass_left as f32 / 3.0).floor() as i32 - 2) > 0 {
        let fuel_needed = (mass_left as f32 / 3.0).floor() as i32 - 2;
        mass_left = fuel_needed;
        fuel_load = fuel_load + fuel_needed;
    }

    fuel_load
}

pub fn run(input: &Vec<i32>) {

    let mut fuel = 0;

    for v in input.iter() {
        fuel = fuel + rocket_equation(*v);
    }

    println!("Rocket equation fuel is = {}", fuel);
}
