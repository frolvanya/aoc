use std::{collections::HashMap, fs};

fn get_antinodes_test(
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    x_bound: usize,
    y_bound: usize,
    part1: bool,
) -> Vec<(i32, i32)> {
    let dx = x2 as i32 - x1 as i32;
    let dy = y2 as i32 - y1 as i32;

    let mut antinodes = Vec::new();

    let mut calculate_antinodes = |mut x: i32, mut y: i32, dx: i32, dy: i32| {
        while x + dx >= 0 && x + dx < x_bound as i32 && y + dy >= 0 && y + dy < y_bound as i32 {
            x += dx;
            y += dy;
            antinodes.push((x, y));
            if part1 {
                break;
            }
        }
    };

    calculate_antinodes(x1 as i32, y1 as i32, -dx, -dy);
    calculate_antinodes(x2 as i32, y2 as i32, dx, dy);

    antinodes
}

fn main() {
    // Part 1
    let input = fs::read_to_string("input.txt").unwrap();

    let mut grid = Vec::new();
    let mut antennas = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        let row = line.chars().collect::<Vec<_>>();
        grid.push(row.clone());
        for (j, col) in row.into_iter().enumerate() {
            if col != '.' {
                antennas.entry(col).or_insert(Vec::new()).push((i, j));
            }
        }
    }

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    for antenna in antennas.values() {
        for i in 0..antenna.len() {
            for j in i + 1..antenna.len() {
                let (x1, y1) = antenna[i];
                let (x2, y2) = antenna[j];

                let antinodes = get_antinodes_test(x1, y1, x2, y2, grid.len(), grid[0].len(), true);

                for antinode in &antinodes[..antinodes.len().min(2)] {
                    visited[antinode.0 as usize][antinode.1 as usize] = true;
                }
            }
        }
    }

    println!("{:?}", visited.iter().flatten().filter(|&&x| x).count());

    // Part 2
    let input = fs::read_to_string("input.txt").unwrap();

    let mut grid = Vec::new();
    let mut antennas = HashMap::new();

    let mut ans = 0;

    for (i, line) in input.lines().enumerate() {
        let row = line.chars().collect::<Vec<_>>();
        grid.push(row.clone());
        for (j, col) in row.into_iter().enumerate() {
            if col != '.' {
                antennas.entry(col).or_insert(Vec::new()).push((i, j));
                ans += 1;
            }
        }
    }

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    for antenna in antennas.values() {
        for i in 0..antenna.len() {
            for j in i + 1..antenna.len() {
                let (x1, y1) = antenna[i];
                let (x2, y2) = antenna[j];

                let antinodes =
                    get_antinodes_test(x1, y1, x2, y2, grid.len(), grid[0].len(), false);

                for antinode in antinodes {
                    if !visited[antinode.0 as usize][antinode.1 as usize]
                        && grid[antinode.0 as usize][antinode.1 as usize] == '.'
                    {
                        visited[antinode.0 as usize][antinode.1 as usize] = true;
                    }
                }
            }
        }
    }

    println!(
        "{:?}",
        ans + visited.iter().flatten().filter(|&&x| x).count()
    );
}
