use crate::Problem;
use std::str::FromStr;
use std::collections::HashSet;

pub struct Problem3 {
    input: Vec<Vec<Operation>>,
}

// One wire per line of text as input
#[derive(PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Operation {
    direction: Direction,
    length: i32,
}
impl FromStr for Operation {

    type Err = std::num::ParseIntError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {

        let direction = match value[0..1].as_ref() {
            "U" => {
                Direction::Up
            },
            "D" => {
                Direction::Down
            },
            "L" => {
                Direction::Left
            },
            "R" => {
                Direction::Right
            }
            _ => { panic!("") }
        };

        let length = value[1..].parse::<i32>().unwrap();

        Ok(Operation{direction, length})
    }
}

// find the intersection point closest to the central port
#[derive(Copy, Clone, Debug)]
struct Coord {
    x: f32,
    y: f32,
}

#[derive(Copy, Clone, Debug)]
struct Segment(Coord, Coord);

impl Segment {
    pub fn intersect(self, other: Self) -> Option<Coord> {

        let a1 = self.1.y - self.0.y;
        let b1 = self.0.x - self.1.x;
        let c1 = a1 * self.0.x + b1 * self.0.y;

        let a2 = other.1.y - other.0.y;
        let b2 = other.0.x - other.1.x;
        let c2 = a2 * other.0.x + b2 * other.0.y;

        let delta = a1 * b2 - a2 * b1;

        if delta == 0.0 {
            return None;
        }

        let x = (b2 * c1 - b1 * c2) / delta;
        let y = (a1 * c2 - a2 * c1) / delta;

        if x == -0.0 || y == -0.0 {
            return None;
        }

        Some(Coord {
            x: (b2 * c1 - b1 * c2) / delta,
            y: (a1 * c2 - a2 * c1) / delta,
        })
    }
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

        let mut lines = Vec::new();

        for line in &self.input {

            let mut position = Coord {x: 0.0, y: 0.0};
            let mut accumulator = Vec::new();

            for operation in line {

                let next = match operation.direction {
                    Direction::Down => {
                        Coord { x: position.x, y: position.y - operation.length as f32 }
                    }
                    Direction::Up => {
                        Coord { x: position.x, y: position.y + operation.length as f32 }
                    }
                    Direction::Left => {
                        Coord { x: position.x - operation.length as f32, y: position.y }
                    }
                    Direction::Right => {
                        Coord { x: position.x + operation.length as f32, y: position.y }
                    }
                };

                accumulator.push(Segment(position,  next));
                position = next;

            }

            lines.push(accumulator);
        }

        let mut intersections = Vec::new();

        for a in lines.first().unwrap() {
            for x in lines.last().unwrap() {

                if let Some(z) = a.intersect(*x) {
                    intersections.push(z);
                    //println!("{:?}", z);
                }

            }
        }

        let mut dist = Vec::new();

        for i in intersections {

            println!("{:?} ==== {}", i, (i.x.abs() + i.y.abs()));
            dist.push((i.x.abs() + i.y.abs()) as i32);
        }

        dist.sort();
        println!("{:?}", dist);
    }

    fn run_part2(&self) {
    }
}

