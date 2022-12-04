use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn fft(signal: Vec<u8>, passes: usize) -> Vec<u8> {
    let mut signal = signal;
    let mut output: Vec<u8>;
    let siglen = signal.len();

    for _ in 0..passes {
        output = signal.clone();

        for (oid, item) in output.iter_mut().enumerate().take(siglen) {
            let mut i = oid;
            let psize = oid + 1;
            let mut res = 0;

            while i < siglen {
                res += &signal[i..min(siglen, psize + i)]
                    .iter()
                    .map(|&x| x as i32)
                    .sum::<i32>();

                i += psize * 2;

                if i >= siglen {
                    break;
                }

                res -= &signal[i..min(siglen, psize + i)]
                    .iter()
                    .map(|&x| x as i32)
                    .sum::<i32>();

                i += psize * 2;
            }

            *item = (res.abs() % 10) as u8;
        }

        signal = output;
    }

    signal
}

fn fft2(signal: Vec<u8>, passes: usize) -> Vec<u8> {
    let mut signal = signal;
    let mut output: Vec<u8>;
    let siglen = signal.len();

    for _ in 0..passes {
        output = signal.clone();

        let mut res: i32 = 0;
        for oid in 0..siglen {
            let i = siglen - oid - 1;

            res += signal[i] as i32;

            output[i] = (res.abs() % 10) as u8;
        }

        signal = output;
    }

    signal
}

fn get_input() -> Vec<u8> {
    let file = match File::open("input/d16.txt") {
        Ok(file) => file,
        Err(error) => panic!("Unable to open file: {:?}", error),
    };

    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let _len = reader.read_line(&mut line);

    line.trim_end()
        .chars()
        .map(|x| match x.to_digit(10) {
            Some(v) => v as u8,
            None => panic!("Unable to parse value: {:?}", x),
        })
        .collect::<Vec<u8>>()
}

fn vec2int(v: Vec<u8>) -> usize {
    let mut res = 0;
    let l = v.len() as u32;

    for i in 0..l {
        res += v[i as usize] as usize * 10usize.pow(l - i - 1);
    }

    res
}

fn main() {
    let signal = get_input();

    let p1 = fft(signal.clone(), 100);
    println!("Part 1: {:?}", vec2int(p1[0..8].to_vec()));

    let siglen = signal.len();
    let mut bigsignal: Vec<u8> = Vec::with_capacity(siglen * 10000);
    for _ in 0..10000 {
        let mut s = signal.clone();
        for _ in 0..siglen {
            bigsignal.append(&mut s);
        }
    }

    let offset = vec2int(signal[0..7].to_vec());
    bigsignal.drain(0..offset);

    let p2 = fft2(bigsignal, 100);
    println!("Part 2: {:?}", vec2int(p2[0..8].to_vec()));
}
