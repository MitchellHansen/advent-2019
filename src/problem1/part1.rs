/*
Fuel required to launch a given module is based on its mass. Specifically, to find the fuel
required for a module, take its mass, divide by three, round down, and subtract 2.

fuel = floor(mass / 3) - 2
*/

pub fn run(input: &Vec<i32>) {

    // fuel = floor(mass / 3) - 2

    let mut fuel = 0;

    for v in input.iter() {
        fuel = fuel + (*v as f32 / 3.0).floor() as i32 - 2;
    }

    println!("Fuel is = {}", fuel);
}
