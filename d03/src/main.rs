use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

fn get_input() -> Vec<Vec<String>> {
    let file = match File::open("input/d03.txt") {
        Ok(file) => file,
        Err(error) => panic!("Unable to open file: {:?}", error),
    };

    let mut values: Vec<Vec<String>> = Vec::new();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        values.push(
            line.split(',')
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        );
    }

    values
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn trace_wire(instructions: Vec<String>) -> Result<Vec<Point>, String> {
    let mut positions: Vec<Point> = Vec::new();

    let mut x = 0;
    let mut y = 0;

    positions.push(Point { x, y });

    for instr in instructions {
        let mut chars = instr.chars();
        let m = match chars.next() {
            Some(c) => c,
            None => return Err("Empty instruction".to_string()),
        };

        let c = match instr[1..].parse::<i32>() {
            Ok(c) => c,
            Err(err) => return Err(format!("{}: {}", err, instr)),
        };

        match m {
            'U' => {
                for i in 0..c {
                    positions.push(Point {
                        x,
                        y: y + (i + 1),
                    });
                }

                y += c;
            }
            'D' => {
                for i in 0..c {
                    positions.push(Point {
                        x,
                        y: y - (i + 1),
                    });
                }

                y -= c;
            }
            'L' => {
                for i in 0..c {
                    positions.push(Point {
                        x: x - (i + 1),
                        y,
                    });
                }

                x -= c;
            }
            'R' => {
                for i in 0..c {
                    positions.push(Point {
                        x: x + (i + 1),
                        y,
                    });
                }

                x += c;
            }
            _ => return Err(format!("Invalid movement: {:?}", m)),
        };
    }

    Ok(positions)
}

fn main() {
    let input = get_input();

    let mut wires: Vec<Vec<Point>> = Vec::new();

    for i in input {
        match trace_wire(i) {
            Ok(w) => wires.push(w),
            Err(err) => panic!("{:?}", err),
        }
    }

    let mut ixn_pts: HashSet<Point> = HashSet::from_iter(wires.get(0).unwrap().iter().cloned());

    for i in 1..wires.len() {
        let s = HashSet::from_iter(wires.get(i).unwrap().iter().cloned());
        let ixn = ixn_pts.intersection(&s);
        ixn_pts = ixn.cloned().collect();
    }

    let distances = ixn_pts
        .iter()
        .map(|p| p.distance())
        .filter(|x| x != &0)
        .collect::<Vec<i32>>();

    let p1 = distances.iter().min().unwrap();

    let mut step_counts: Vec<usize> = Vec::new();
    for p in ixn_pts {
        let steps = wires
            .iter()
            .map(|w| match w.iter().position(|x| x == &p) {
                Some(p) => p,
                None => panic!("Not found: {:?}", p),
            })
            .sum();

        if steps != 0 {
            step_counts.push(steps);
        }
    }

    let p2 = step_counts.iter().min().unwrap();

    println!("Part 1: {:?}\nPart 2: {}", p1, p2);
}
