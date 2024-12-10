use std::{collections::HashMap, fs};

fn main() {
    // Part 1
    let input = fs::read_to_string("input.txt").unwrap();

    let mut map = HashMap::new();
    let mut switch = false;
    let mut updates = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            switch = true;
            continue;
        }

        if switch {
            updates.push(
                line.split(',')
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
            );

            continue;
        }

        let numbers = line
            .split("|")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        map.entry(numbers[0]).or_insert(Vec::new()).push(numbers[1]);
    }

    let mut ans = 0;

    for update in updates {
        let mut valid = true;
        for i in 1..update.len() {
            for j in i - 1..i {
                if let Some(v) = map.get_mut(&update[i]) {
                    for &n in v.iter() {
                        if n == update[j] {
                            valid = false;
                            break;
                        }
                    }
                }
            }
        }

        if valid {
            ans += update[update.len() / 2];
        }
    }

    println!("{}", ans);

    // Part 2
    let input = fs::read_to_string("input.txt").unwrap();

    let mut map = HashMap::new();
    let mut switch = false;
    let mut updates = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            switch = true;
            continue;
        }

        if switch {
            updates.push(
                line.split(',')
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
            );

            continue;
        }

        let numbers = line
            .split("|")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        map.entry(numbers[0]).or_insert(Vec::new()).push(numbers[1]);
    }

    let mut ans = 0;

    for update in updates.iter_mut() {
        let mut invalid = false;
        loop {
            let mut to_swap = (None, None);
            for i in 1..update.len() {
                for j in i - 1..i {
                    if let Some(v) = map.get_mut(&update[i]) {
                        for &n in v.iter() {
                            if n == update[j] {
                                to_swap = (Some(i), Some(j));
                                break;
                            }
                        }
                    }
                }
            }
            if let (Some(i), Some(j)) = to_swap {
                update.swap(i, j);
                invalid = true;
            } else {
                break;
            }
        }

        if invalid {
            ans += update[update.len() / 2];
        }
    }

    println!("{}", ans);
}
