fn is_valid(digits: &Vec<char>) -> bool {
    let mut sorted = digits.clone();
    sorted.sort_unstable();

    if digits != &sorted {
        return false;
    }

    for i in digits.iter() {
        let c = digits.iter().filter(|x| *x == i).count();
        if c > 1 {
            return true;
        }
    }

    false
}

fn contains_one_pair(digits: &[char]) -> bool {
    for i in digits.iter() {
        let c = digits.iter().filter(|x| *x == i).count();
        if c == 2 {
            return true;
        }
    }

    false
}

fn main() {
    let a = 273025;
    let b = 767253;

    let mut p1: u32 = 0;
    let mut p2: u32 = 0;

    for i in a..b + 1 {
        let digits: Vec<char> = i.to_string().chars().collect();

        if is_valid(&digits) {
            p1 += 1;

            if contains_one_pair(&digits) {
                p2 += 1;
            }
        }
    }

    println!("Part 1: {}\nPart 2: {}", p1, p2);
}
