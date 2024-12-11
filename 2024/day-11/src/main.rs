use std::collections::HashMap;

fn change_stone(stone: usize) -> Vec<usize> {
    if stone == 0 {
        vec![1]
    } else if stone.to_string().len() % 2 == 0 {
        let chars = stone.to_string().chars().collect::<Vec<_>>();
        let first_half = chars[..chars.len() / 2].iter().collect::<String>();
        let second_half = chars[chars.len() / 2..].iter().collect::<String>();
        return vec![first_half.parse().unwrap(), second_half.parse().unwrap()];
    } else {
        vec![stone * 2024]
    }
}

fn main() {
    // Part 1
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut stones = input
        .split_ascii_whitespace()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..25 {
        let mut new_stones = Vec::new();

        for stone in stones {
            new_stones.append(&mut change_stone(stone));
        }

        stones = new_stones;
    }

    println!("{}", stones.len());

    // Part 2
    let input = std::fs::read_to_string("input.txt").unwrap();

    let stones = input
        .split_ascii_whitespace()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut ans = 0;

    for stone in stones {
        let mut stone_counts: HashMap<usize, usize> = HashMap::new();
        stone_counts.insert(stone, 1);

        for _ in 0..75 {
            let mut new_counts = HashMap::new();

            for (&stone, &count) in &stone_counts {
                for new_stone in change_stone(stone) {
                    *new_counts.entry(new_stone).or_insert(0) += count;
                }
            }

            stone_counts = new_counts;
        }

        ans += stone_counts.values().sum::<usize>();
    }

    println!("{}", ans);
}
