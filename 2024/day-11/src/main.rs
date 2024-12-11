use std::collections::HashMap;

fn change_stone(stone: usize) -> Vec<usize> {
    if stone == 0 {
        vec![1]
    } else if stone.to_string().len() % 2 == 0 {
        let chars = stone.to_string().chars().collect::<Vec<_>>();

        let first_half = chars[..chars.len() / 2].iter().collect::<String>();
        let second_half = chars[chars.len() / 2..].iter().collect::<String>();

        vec![first_half.parse().unwrap(), second_half.parse().unwrap()]
    } else {
        vec![stone * 2024]
    }
}

fn solve(mut stones: HashMap<usize, usize>, blinks: usize) -> usize {
    for _ in 0..blinks {
        let mut new_stones = HashMap::new();

        for (stone, amount) in stones {
            for new_stone in change_stone(stone) {
                *new_stones.entry(new_stone).or_insert(0) += amount;
            }
        }

        stones = new_stones;
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
