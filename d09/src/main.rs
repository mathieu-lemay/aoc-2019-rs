use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Interrupt {
    WaitingOnInput,
}

struct IntCodeCPU {
    intcodes: Vec<i64>,
    ip: usize,
    halted: bool,

    input: Option<i64>,
    output: Vec<i64>,

    modes: Vec<i64>,
    rel_offset: i64,
}

impl IntCodeCPU {
    fn build(intcodes: Vec<i64>, input: Option<i64>) -> IntCodeCPU {
        IntCodeCPU {
            intcodes,
            ip: 0,
            halted: false,
            input,
            output: Vec::new(),
            modes: Vec::new(),
            rel_offset: 0,
        }
    }

    fn run(&mut self) -> Result<i64, Interrupt> {
        self.intcodes.resize(2000, 0);
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
                    return Ok(self._ld(0, None));
                }
                _ => {
                    println!("{:?}", self.intcodes);
                    panic!(format!("Invalid op: {:?} (ip={:?})", op, self.ip));
                }
            };
        }

        panic!("Reached end of opcodes");
    }

    fn _get_instruction(&self) -> (i64, Vec<i64>) {
        let mut instr = self.intcodes[self.ip];
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
        return &self.intcodes[self.ip + 1..self.ip + 1 + n];
    }

    fn add(&mut self) {
        let params = self._get_params(3);

        let v1 = self._ld(params[0], self.modes.get(0));
        let v2 = self._ld(params[1], self.modes.get(1));
        let out = params[2];

        let addr = self._addr(out, self.modes.get(2));

        self.intcodes[addr] = v1 + v2;
    }

    fn mul(&mut self) {
        let params = self._get_params(3);

        let v1 = self._ld(params[0], self.modes.get(0));
        let v2 = self._ld(params[1], self.modes.get(1));
        let out = params[2];

        let addr = self._addr(out, self.modes.get(2));

        self.intcodes[addr] = v1 * v2;
    }

    fn read(&mut self) -> Option<Interrupt> {
        let params = self._get_params(1);

        match self.input {
            Some(v) => {
                let addr = self._addr(params[0], self.modes.get(0));
                self.intcodes[addr] = v;
                None
            }
            None => Some(Interrupt::WaitingOnInput),
        }
    }

    fn write(&mut self) {
        let params = self._get_params(1);

        let v = self._ld(params[0], self.modes.get(0));

        self.output.push(v);
    }

    fn jump_if_true(&mut self) {
        let params = self._get_params(2);

        let v1 = self._ld(params[0], self.modes.get(0));
        let v2 = self._ld(params[1], self.modes.get(1));

        if v1 != 0 {
            self.ip = v2 as usize;
        } else {
            self.ip += 3;
        }
    }

    fn jump_if_false(&mut self) {
        let params = self._get_params(2);

        let v1 = self._ld(params[0], self.modes.get(0));
        let v2 = self._ld(params[1], self.modes.get(1));

        if v1 == 0 {
            self.ip = v2 as usize;
        } else {
            self.ip += 3;
        }
    }

    fn lt(&mut self) {
        let params = self._get_params(3);

        let v1 = self._ld(params[0], self.modes.get(0));
        let v2 = self._ld(params[1], self.modes.get(1));

        let addr = self._addr(params[2], self.modes.get(2));

        if v1 < v2 {
            self.intcodes[addr] = 1;
        } else {
            self.intcodes[addr] = 0;
        }
    }

    fn eq(&mut self) {
        let params = self._get_params(3);

        let v1 = self._ld(params[0], self.modes.get(0));
        let v2 = self._ld(params[1], self.modes.get(1));

        let addr = self._addr(params[2], self.modes.get(2));

        if v1 == v2 {
            self.intcodes[addr] = 1;
        } else {
            self.intcodes[addr] = 0;
        }
    }

    fn rel(&mut self) {
        let params = self._get_params(1);

        let v = self._ld(params[0], self.modes.get(0));

        self.rel_offset += v;
    }

    fn _ld(&self, addr: i64, mode: Option<&i64>) -> i64 {
        match mode.unwrap_or(&0) {
            0 => self.intcodes[addr as usize],
            1 => addr,
            2 => self.intcodes[(addr + self.rel_offset) as usize],
            _ => panic!("Invalid mode: {:?}", mode),
        }
    }

    fn _addr(&self, addr: i64, mode: Option<&i64>) -> usize {
        let addr = match mode.unwrap_or(&0) {
            0 => addr,
            2 => addr + self.rel_offset,
            _ => panic!("Invalid mode: {:?}", mode),
        };

        addr as usize
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

fn main() {
    let input = get_input();

    let mut cpu = IntCodeCPU::build(input.clone(), Some(1));

    let res = cpu.run();
    if res.is_err() {
        panic!(format!(
            "Program stopped with interrupt {:?}",
            res.err().unwrap()
        ));
    }

    match cpu.output.pop() {
        Some(v) => println!("Part 1: {}", v),
        None => panic!("Program didn't produce output."),
    }

    cpu = IntCodeCPU::build(input.clone(), Some(2));

    let res = cpu.run();
    if res.is_err() {
        panic!(format!(
            "Program stopped with interrupt {:?}",
            res.err().unwrap()
        ));
    }

    match cpu.output.pop() {
        Some(v) => println!("Part 2: {}", v),
        None => panic!("Program didn't produce output."),
    }
}
