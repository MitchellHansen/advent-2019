use crate::Problem;
use std::str::FromStr;
use std::collections::HashSet;
use std::thread::yield_now;

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
    pub fn len(self) -> i32 {
        return ((self.1.x - self.0.x).powi(2) + (self.1.y - self.0.y).powi(2)).sqrt() as i32
    }
    pub fn intersect(self, other: Self) -> Option<Coord> {

        // First line is horiz and second it vert
        if self.0.x == self.1.x && other.0.y == other.1.y {
            let mut x_points = vec![self.0.x as i32, other.0.x as i32, other.1.x as i32];
            let mut y_points = vec![self.0.y as i32, self.1.y as i32, other.0.y as i32];
            x_points.sort(); y_points.sort();
            if x_points[1] == self.0.x as i32 && y_points[1] == other.0.y as i32 {
                Some(Coord {
                    x: x_points[1] as f32,
                    y: y_points[1] as f32,
                })
            } else {
                None
            }
        }
        // First line is vert and second is horiz
        else if self.0.y == self.1.y && other.0.x == other.1.x {
            let mut x_points = vec![other.0.x as i32, self.0.x as i32, self.1.x as i32];
            let mut y_points = vec![other.0.y as i32, other.1.y as i32, self.0.y as i32];
            x_points.sort(); y_points.sort();
            if x_points[1] == other.0.x as i32 && y_points[1] == self.0.y as i32 {
                Some(Coord {
                    x: x_points[1] as f32,
                    y: y_points[1] as f32,
                })
            } else {
                None
            }
        }
        else {
            None
        }
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
                    if z.x == 0.0 || z.y == 0.0 {

                    } else {
                        intersections.push(z);
                    }

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

        let mut a_accu = 0;
        let mut x_accu = 0;

        for a in lines.first().unwrap() {
            x_accu = 0;
            a_accu += a.len();
            for x in lines.last().unwrap() {
                x_accu += x.len();

                if let Some(z) = a.intersect(*x) {
                    if z.x == 0.0 || z.y == 0.0 {

                    } else {
                        intersections.push(
                            (
                                z,
                                x_accu - Segment(x.1, z).len() +
                                a_accu - Segment(a.1, z).len()
                            )
                        );
                    }

                }
            }
        }

        let mut dist = Vec::new();

        for (i, l) in intersections {

            println!("{:?} ==== {} ==== {}", i, l, (i.x.abs() + i.y.abs()));
            dist.push((i.x.abs() + i.y.abs()) as i32);
        }

        dist.sort();
        println!("{:?}", dist);
    }
}

