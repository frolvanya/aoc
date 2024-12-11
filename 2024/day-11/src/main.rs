use std::collections::HashMap;

enum Stone {
    Replaced(usize),
    Splitted(usize, usize),
}

fn change_stone(stone: usize) -> Stone {
    if stone == 0 {
        return Stone::Replaced(1);
    }

    let num_digits = stone.ilog10() as usize + 1;
    if num_digits % 2 == 0 {
        let mid = num_digits / 2;

        let divisor = 10_usize.pow(mid as u32);
        let first_stone = stone / divisor;
        let second_stone = stone % divisor;

        return Stone::Splitted(first_stone, second_stone);
    }

    Stone::Replaced(stone * 2024)
}

fn solve(mut stones: HashMap<usize, usize>, blinks: usize) -> usize {
    let mut new_stones = HashMap::new();

    for _ in 0..blinks {
        for (&stone, &amount) in &stones {
            let new_stone = change_stone(stone);
            match new_stone {
                Stone::Replaced(replacement) => {
                    *new_stones.entry(replacement).or_insert(0) += amount;
                }
                Stone::Splitted(first_stone, second_stone) => {
                    *new_stones.entry(first_stone).or_insert(0) += amount;
                    *new_stones.entry(second_stone).or_insert(0) += amount;
                }
            }
        }

        std::mem::swap(&mut stones, &mut new_stones);
        new_stones.clear();
    }

    stones.values().sum()
}

fn main() {
    // Part 1
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut stones = HashMap::new();

    for stone in input
        .split_ascii_whitespace()
        .map(|line| line.parse::<usize>().unwrap())
    {
        *stones.entry(stone).or_insert(0) += 1;
    }

    println!("{}", solve(stones.clone(), 25));

    // Part 2
    println!("{}", solve(stones, 75));
}
