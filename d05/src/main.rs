use intcode::IntCodeCPU;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_input() -> Vec<i64> {
    let file = match File::open("input/d05.txt") {
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

fn main() {
    let program = get_input();

    let mut cpu = IntCodeCPU::build(program.clone(), vec![1]);
    cpu.run().expect("Error running program");
    let mut output = cpu.pop_output();
    println!(">>> {:?}", output);
    println!("Part 1: {:?}", output.pop().unwrap());

    let mut cpu = IntCodeCPU::build(program, vec![5]);
    cpu.run().expect("Error running program");
    println!("Part 2: {:?}", cpu.pop_output().pop().unwrap());
}
