use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::slice::Iter;

struct IntCodeRunner {
    intcodes: Vec<i32>,
    input: Option<i32>,
    ip: usize,
}

impl IntCodeRunner {
    fn run(&mut self) -> Result<i32, String> {
        let mut ic: i32;
        let mut op: i32;

        while self.ip < self.intcodes.len() {
            ic = self._get_next_intcode();

            op = self._decode_op(ic);
            let modes = self._decode_modes(ic);
            let mut modes = modes.iter();

            match op {
                1 => {
                    let p1 = self._get_next_intcode();
                    let p2 = self._get_next_intcode();
                    let out = self._get_next_intcode();

                    let v1 = self._ld(p1, modes.next());
                    let v2 = self._ld(p2, modes.next());

                    self._st(out, v1 + v2);
                }
                2 => {
                    let p1 = self._get_next_intcode();
                    let p2 = self._get_next_intcode();
                    let out = self._get_next_intcode();

                    let v1 = self._ld(p1, modes.next());
                    let v2 = self._ld(p2, modes.next());

                    self._st(out, v1 * v2);
                }
                3 => {
                    let out = self._get_next_intcode();

                    let v = self._read();

                    self._st(out, v);
                }
                4 => {
                    let p1 = self._get_next_intcode();

                    println!("<<< {}", self._ld(p1, modes.next()));
                }
                5 => {
                    let p1 = self._get_next_intcode();
                    let p2 = self._get_next_intcode();

                    let v1 = self._ld(p1, modes.next());
                    let v2 = self._ld(p2, modes.next());

                    if v1 != 0 {
                        self.ip = v2 as usize;
                    }
                }
                6 => {
                    let p1 = self._get_next_intcode();
                    let p2 = self._get_next_intcode();

                    let v1 = self._ld(p1, modes.next());
                    let v2 = self._ld(p2, modes.next());

                    if v1 == 0 {
                        self.ip = v2 as usize;
                    }
                }
                7 => {
                    let p1 = self._get_next_intcode();
                    let p2 = self._get_next_intcode();
                    let out = self._get_next_intcode();

                    let v1 = self._ld(p1, modes.next());
                    let v2 = self._ld(p2, modes.next());

                    if v1 < v2 {
                        self._st(out, 1);
                    } else {
                        self._st(out, 0);
                    }
                }
                8 => {
                    let p1 = self._get_next_intcode();
                    let p2 = self._get_next_intcode();
                    let out = self._get_next_intcode();

                    let v1 = self._ld(p1, modes.next());
                    let v2 = self._ld(p2, modes.next());

                    if v1 == v2 {
                        self._st(out, 1);
                    } else {
                        self._st(out, 0);
                    }
                }
                99 => return Ok(self._ld(0, None)),
                _ => return Err(format!("Invalid op: {:?} (ic={:?})", op, ic)),
            };
        }

        return Err("Reached end of opcodes".to_string());
    }

    fn _get_next_intcode(&mut self) -> i32 {
        let op = self.intcodes[self.ip];
        self.ip += 1;

        return op;
    }

    fn _decode_op(&mut self, ic: i32) -> i32 {
        return ic % 100;
    }

    fn _decode_modes(&mut self, ic: i32) -> Vec<i32> {
        let mut modes = Vec::new();
        let mut ic = ic / 100;

        while ic > 0 {
            modes.push(ic % 10);
            ic /= 10;
        }

        return modes;
    }

    fn _ld(&self, addr: i32, mode: Option<&i32>) -> i32 {
        match mode.unwrap_or(&0) {
            0 => self.intcodes[addr as usize],
            1 => addr,
            _ => panic!("Invalid mode: {:?}", mode),
        }
    }

    fn _st(&mut self, addr: i32, val: i32) {
        self.intcodes[addr as usize] = val;
    }

    fn _read(&self) -> i32 {
        match self.input {
            Some(v) => return v,
            None => {
                print!(">>> ");
                io::stdout().flush().expect("Flush failed");

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("error: unable to read user input");

                return input
                    .trim_end()
                    .parse::<i32>()
                    .expect("Unable to parse input");
            }
        }
    }
}

fn get_input() -> Vec<i32> {
    let file = match File::open("../input/d05.txt") {
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

    let mut icr = IntCodeRunner {
        intcodes: input.clone(),
        input: Some(1),
        ip: 0,
    };

    println!("=== Part 1 ===");
    icr.run().expect("Error running program");
    println!("=== Part 1 ===\n");

    let mut icr = IntCodeRunner {
        intcodes: input.clone(),
        input: Some(5),
        ip: 0,
    };

    println!("=== Part 2 ===");
    icr.run().expect("Error running program");
    println!("=== Part 2 ===\n");
}
