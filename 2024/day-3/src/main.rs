use std::io::Read;

fn is_enable(sequence: &[char], i: usize) -> Option<(bool, usize)> {
    if i + 7 < sequence.len() && sequence[i..i + 7] == ['d', 'o', 'n', '\'', 't', '(', ')'] {
        return Some((false, 7));
    } else if i + 4 < sequence.len() && sequence[i..i + 4] == ['d', 'o', '(', ')'] {
        return Some((true, 4));
    }

    None
}

fn is_mul(sequence: &[char], i: usize) -> Option<(usize, usize)> {
    if i + 4 < sequence.len() && sequence[i..i + 4] == ['m', 'u', 'l', '('] {
        let mut j = i + 4;

        let mut first_number = String::new();
        while j < sequence.len() && sequence[j].is_ascii_digit() {
            first_number.push(sequence[j]);
            j += 1;
        }

        if sequence[j] != ',' {
            return None;
        }
        j += 1;

        let mut second_number = String::new();
        while j < sequence.len() && sequence[j].is_ascii_digit() {
            second_number.push(sequence[j]);
            j += 1;
        }

        if sequence[j] != ')' {
            return None;
        }

        return Some((
            first_number.parse::<usize>().unwrap() * second_number.parse::<usize>().unwrap(),
            j + 1,
        ));
    }

    None
}

fn main() {
    // Part 1
    let mut input = String::new();
    let mut file = std::fs::File::open("input.txt").unwrap();

    file.read_to_string(&mut input).unwrap();

    let sequence = input.trim().chars().collect::<Vec<_>>();

    let mut ans = 0;
    let mut i = 0;
    while i < sequence.len() {
        if let Some((res, next)) = is_mul(&sequence, i) {
            ans += res;
            i = next;
        } else {
            i += 1;
        }
    }

    println!("{:?}", ans);

    // Part 2
    let mut input = String::new();
    let mut file = std::fs::File::open("input.txt").unwrap();

    file.read_to_string(&mut input).unwrap();

    let sequence = input.trim().chars().collect::<Vec<_>>();

    let mut enable = true;
    let mut ans = 0;
    let mut i = 0;
    while i < sequence.len() {
        if let Some((res, next)) = is_enable(&sequence, i) {
            enable = res;
            i += next;
            continue;
        }

        if enable {
            if let Some((res, next)) = is_mul(&sequence, i) {
                ans += res;
                i = next;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    println!("{:?}", ans);
}
