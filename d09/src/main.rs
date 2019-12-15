use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Interrupt {
    WaitingOnInput,
}

struct IntCodeCPU {
    memory: Vec<i64>,
    ip: usize,
    halted: bool,

    input: Vec<i64>,
    output: Vec<i64>,

    modes: Vec<i64>,
    rel_offset: i64,
}

impl IntCodeCPU {
    fn build(memory: Vec<i64>, input: Vec<i64>) -> IntCodeCPU {
        IntCodeCPU {
            memory,
            ip: 0,
            halted: false,
            input,
            output: Vec::new(),
            modes: Vec::new(),
            rel_offset: 0,
        }
    }

    fn run(&mut self) -> Result<i64, Interrupt> {
        while !self.halted {
            let instr = self._get_instruction();

            let op = instr.0;
            self.modes = instr.1;

            match op {
                1 => {
                    self.add();
                    self.ip += 4;
                }
                2 => {
                    self.mul();
                    self.ip += 4;
                }
                3 => {
                    match self.read() {
                        Some(int) => return Err(int),
                        None => {}
                    };
                    self.ip += 2;
                }
                4 => {
                    self.write();
                    self.ip += 2;
                }
                5 => {
                    self.jump_if_true();
                }
                6 => {
                    self.jump_if_false();
                }
                7 => {
                    self.lt();
                    self.ip += 4;
                }
                8 => {
                    self.eq();
                    self.ip += 4;
                }
                9 => {
                    self.rel();
                    self.ip += 2;
                }
                99 => {
                    self.halted = true;
                    return Ok(self.peek(0));
                }
                _ => {
                    println!("{:?}", self.memory);
                    panic!(format!("Invalid op: {:?} (ip={:?})", op, self.ip));
                }
            };
        }

        panic!("Reached end of opcodes");
    }

    fn _get_instruction(&self) -> (i64, Vec<i64>) {
        let mut instr = self.memory[self.ip];
        let mut modes: Vec<i64> = Vec::new();

        let op = instr % 100;

        instr /= 100;

        while instr > 0 {
            modes.push(instr % 10);
            instr /= 10;
        }

        return (op, modes);
    }

    fn _get_params(&self, n: usize) -> &[i64] {
        return &self.memory[self.ip + 1..self.ip + 1 + n];
    }

    fn add(&mut self) {
        let params = self._get_params(3);

        let v1 = self.load(params[0], self.modes.get(0));
        let v2 = self.load(params[1], self.modes.get(1));

        let tgt_addr = self._resolve_addr(params[2], self.modes.get(2));

        self.poke(tgt_addr, v1 + v2);
    }

    fn mul(&mut self) {
        let params = self._get_params(3);

        let v1 = self.load(params[0], self.modes.get(0));
        let v2 = self.load(params[1], self.modes.get(1));

        let tgt_addr = self._resolve_addr(params[2], self.modes.get(2));

        self.poke(tgt_addr, v1 * v2);
    }

    fn read(&mut self) -> Option<Interrupt> {
        let params = self._get_params(1);

        if self.input.len() == 0 {
            return Some(Interrupt::WaitingOnInput);
        }

        let addr = self._resolve_addr(params[0], self.modes.get(0));
        let v = self.input[0];
        self.poke(addr, v);

        self.input.remove(0);

        return None;
    }

    fn write(&mut self) {
        let params = self._get_params(1);

        let v = self.load(params[0], self.modes.get(0));

        self.output.push(v);
    }

    fn jump_if_true(&mut self) {
        let params = self._get_params(2);

        let v1 = self.load(params[0], self.modes.get(0));
        let v2 = self.load(params[1], self.modes.get(1));

        if v1 != 0 {
            self.ip = v2 as usize;
        } else {
            self.ip += 3;
        }
    }

    fn jump_if_false(&mut self) {
        let params = self._get_params(2);

        let v1 = self.load(params[0], self.modes.get(0));
        let v2 = self.load(params[1], self.modes.get(1));

        if v1 == 0 {
            self.ip = v2 as usize;
        } else {
            self.ip += 3;
        }
    }

    fn lt(&mut self) {
        let params = self._get_params(3);

        let v1 = self.load(params[0], self.modes.get(0));
        let v2 = self.load(params[1], self.modes.get(1));

        let addr = self._resolve_addr(params[2], self.modes.get(2));

        if v1 < v2 {
            self.poke(addr, 1);
        } else {
            self.poke(addr, 0);
        }
    }

    fn eq(&mut self) {
        let params = self._get_params(3);

        let v1 = self.load(params[0], self.modes.get(0));
        let v2 = self.load(params[1], self.modes.get(1));

        let addr = self._resolve_addr(params[2], self.modes.get(2));

        if v1 == v2 {
            self.poke(addr, 1);
        } else {
            self.poke(addr, 0);
        }
    }

    fn rel(&mut self) {
        let params = self._get_params(1);

        let v = self.load(params[0], self.modes.get(0));

        self.rel_offset += v;
    }

    fn load(&self, addr: i64, mode: Option<&i64>) -> i64 {
        match mode.unwrap_or(&0) {
            0 | 2 => {
                return self.peek(self._resolve_addr(addr, mode));
            }
            1 => addr,
            _ => panic!("Invalid mode: {:?}", mode),
        }
    }

    fn peek(&self, addr: usize) -> i64 {
        self.memory[addr]
    }

    fn poke(&mut self, addr: usize, value: i64) {
        if addr >= self.memory.len() {
            self.memory.resize(addr + 1, 0);
        }

        self.memory[addr] = value;
    }

    fn _resolve_addr(&self, addr: i64, mode: Option<&i64>) -> usize {
        let addr = match mode.unwrap_or(&0) {
            0 => addr,
            2 => addr + self.rel_offset,
            _ => panic!("Invalid mode: {:?}", mode),
        };

        addr as usize
    }

    fn pop_output(&mut self) -> Vec<i64> {
        let output = self.output.clone();

        self.output.clear();

        return output;
    }
}

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
