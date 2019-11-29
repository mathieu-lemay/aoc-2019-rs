use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_fuel_req(weight: i32) -> i32 {
    (((weight as f32) / 3.0).floor() as i32) - 2
}

fn get_fuel_req_recur(weight: i32) -> i32 {
    let fuel = (((weight as f32) / 3.0).floor() as i32) - 2;

    if fuel > 0 {
        fuel + get_fuel_req_recur(fuel)
    } else {
        0
    }
}

fn get_input() -> Vec<i32> {
    let mut values: Vec<i32> = Vec::new();

    let file = match File::open("../input/d01.txt") {
        Ok(file) => file,
        Err(error) => panic!("Unable to open file: {:?}", error),
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        values.push(match line.parse::<i32>() {
            Ok(val) => val,
            Err(err) => panic!("Invalid input value: {:?}: {:?}", line, err),
        });
    }

    values
}

fn main() {
    let input = get_input();
    let mut s1: i32 = 0;
    let mut s2: i32 = 0;

    for i in input {
        s1 += get_fuel_req(i);
        s2 += get_fuel_req_recur(i);
    }

    println!("Part 1: {}\nPart 2: {}", s1, s2);
}
