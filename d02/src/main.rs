use intcode::IntCodeCPU;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_input() -> Vec<i64> {
    let file = match File::open("input/d02.txt") {
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
    let input = get_input();
    let input2 = 19690720;

    let mut cpu = IntCodeCPU::build(input.clone(), Vec::new());
    cpu.poke(1, 12);
    cpu.poke(2, 2);
    let p1 = match cpu.run() {
        Ok(val) => val,
        Err(err) => panic!("{:?}", err),
    };

    println!("Part 1: {}", p1);

    let mut p2 = 0;

    for n in 0..100 {
        for v in 0..100 {
            let mut cpu = IntCodeCPU::build(input.clone(), Vec::new());
            cpu.poke(1, n);
            cpu.poke(2, v);
            let res = match cpu.run() {
                Ok(val) => val,
                Err(err) => panic!("{:?}", err),
            };

            if res == input2 {
                p2 = 100 * n + v;
                break;
            }
        }
    }

    println!("Part 2: {}", p2);
}
