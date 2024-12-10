use std::{collections::HashMap, io::Read};

fn main() {
    // Part 1
    let mut input = String::new();
    let mut file = std::fs::File::open("input.txt").unwrap();

    file.read_to_string(&mut input).unwrap();

    let mut numbers_left = Vec::new();
    let mut numbers_right = Vec::new();

    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        numbers_left.push(numbers[0]);
        numbers_right.push(numbers[1]);
    }

    numbers_left.sort_unstable();
    numbers_right.sort_unstable();

    let ans = numbers_left
        .iter()
        .zip(numbers_right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();

    println!("{}", ans);

    // Part 2
    let mut input = String::new();
    let mut file = std::fs::File::open("input.txt").unwrap();

    file.read_to_string(&mut input).unwrap();

    let mut numbers_left = Vec::new();
    let mut map = HashMap::new();

    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        numbers_left.push(numbers[0]);
        *map.entry(numbers[1]).or_insert(0) += 1;
    }

    let mut ans = 0;

    for num in numbers_left {
        ans += num * map.get(&num).unwrap_or(&0);
    }

    println!("{}", ans);
}
