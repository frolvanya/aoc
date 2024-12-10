use std::io::Read;

fn is_safe(vec: &[i32], removed: Option<usize>) -> bool {
    let mut valid = [true, true];
    for i in 1..vec.len() {
        if Some(i) == removed {
            continue;
        }

        let prev = if removed == Some(i - 1) {
            if i == 1 {
                continue;
            }
            vec[i - 2]
        } else {
            vec[i - 1]
        };

        if vec[i] < prev {
            valid[0] = false;
        }

        if vec[i] > prev {
            valid[1] = false;
        }

        let diff = (vec[i] - prev).abs();
        if !(1..=3).contains(&diff) {
            valid[0] = false;
            valid[1] = false;
        }

        if !valid[0] && !valid[1] {
            break;
        }
    }

    valid[0] || valid[1]
}

fn main() {
    // Part 1
    let mut input = String::new();
    let mut file = std::fs::File::open("input.txt").unwrap();

    file.read_to_string(&mut input).unwrap();

    let mut ans = 0;
    for line in input.lines() {
        let vec = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if is_safe(&vec, None) {
            ans += 1;
        }
    }

    println!("{}", ans);

    // Part 2
    let mut input = String::new();
    let mut file = std::fs::File::open("input.txt").unwrap();

    file.read_to_string(&mut input).unwrap();

    let mut ans = 0;
    for line in input.lines() {
        let vec = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        for i in 0..vec.len() {
            if is_safe(&vec, Some(i)) {
                ans += 1;
                break;
            }
        }
    }

    println!("{}", ans);
}
