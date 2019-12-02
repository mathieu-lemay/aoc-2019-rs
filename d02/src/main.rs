use std::fs::File;
use std::io::{BufRead, BufReader};

fn run_opcodes(opcodes: &mut Vec<i32>, n: i32, v: i32) -> Result<i32, String> {
    let mut ip: usize = 0;
    let mut op: i32;

    op = opcodes[ip];

    opcodes[1] = n;
    opcodes[2] = v;

    while ip < opcodes.len() {
        let in1 = opcodes[ip + 1] as usize;
        let in2 = opcodes[ip + 2] as usize;
        let out = opcodes[ip + 3] as usize;

        let val = match op {
            1 => opcodes[in1] + opcodes[in2],
            2 => opcodes[in1] * opcodes[in2],
            99 => return Ok(opcodes[0]),
            _ => return Err(format!("Invalid op: {:?}", op)),
        };

        opcodes[out] = val;

        ip += 4;
        op = opcodes[ip];
    }

    return Err("Reached end of opcodes".to_string());
}

fn get_input() -> Vec<i32> {
    let file = match File::open("../input/d02.txt") {
        Ok(file) => file,
        Err(error) => panic!("Unable to open file: {:?}", error),
    };

    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let _len = reader.read_line(&mut line);

    line.trim_end()
        .split(',')
        .map(|x| match x.parse::<i32>() {
            Ok(v) => v,
            Err(err) => panic!("Unable to parse value: {:?}: {:?}", x, err),
        })
        .collect::<Vec<i32>>()
}

fn main() {
    let input = get_input();
    let input2 = 19690720;

    let p1 = match run_opcodes(&mut input.clone(), 12, 2) {
        Ok(val) => val,
        Err(err) => panic!(err),
    };

    let mut p2 = 0;

    for n in 0..100 {
        for v in 0..100 {
            let res = match run_opcodes(&mut input.clone(), n, v) {
                Ok(val) => val,
                Err(err) => panic!(err),
            };

            if res == input2 {
                p2 = 100 * n + v;
                break;
            }
        }
    }

    println!("Part 1: {}\nPart 2: {}", p1, p2);
}
