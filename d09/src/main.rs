use intcode::IntCodeCPU;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_input() -> Vec<i64> {
    let file = match File::open("../input/d09.txt") {
        Ok(file) => file,
        Err(error) => panic!("Unable to open file: {:?}", error),
    };

    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let _len = reader.read_line(&mut line);

    line.trim_end()
        .split(',')
        .map(|x| match x.parse::<i64>() {
            Ok(v) => v,
            Err(err) => panic!("Unable to parse value: {:?}: {:?}", x, err),
        })
        .collect::<Vec<i64>>()
}

fn run_program(program: &Vec<i64>, input: Vec<i64>) -> Vec<i64> {
    let mut cpu = IntCodeCPU::build(program.clone(), input);

    let res = cpu.run();
    if res.is_err() {
        panic!(format!(
            "Program stopped with interrupt {:?}",
            res.err().unwrap()
        ));
    }

    return cpu.pop_output();
}

fn main() {
    let program = get_input();

    let mut output = run_program(&program, vec![1]);
    match output.pop() {
        Some(v) => println!("Part 1: {}", v),
        None => panic!("Program didn't produce output."),
    }

    let mut output = run_program(&program, vec![2]);
    match output.pop() {
        Some(v) => println!("Part 2: {}", v),
        None => panic!("Program didn't produce output."),
    }
}
