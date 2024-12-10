use std::collections::HashSet;

fn dfs1(
    map: &Vec<Vec<u32>>,
    i: usize,
    j: usize,
    curr: &mut i32,
    visited: &mut HashSet<(usize, usize)>,
) {
    if map[i][j] == 9 {
        if visited.insert((i, j)) {
            *curr += 1;
        }
        return;
    }

    let i = i as i32;
    let j = j as i32;

    for (ni, nj) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
        if ni < 0 || nj < 0 || ni >= map.len() as i32 || nj >= map[0].len() as i32 {
            continue;
        }
        if map[ni as usize][nj as usize] != map[i as usize][j as usize] + 1 {
            continue;
        }
        dfs1(map, ni as usize, nj as usize, curr, visited);
    }
}

fn dfs2(map: &Vec<Vec<u32>>, i: usize, j: usize, curr: &mut i32) {
    if map[i][j] == 9 {
        *curr += 1;
        return;
    }

    let i = i as i32;
    let j = j as i32;

    for (ni, nj) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
        if ni < 0 || nj < 0 || ni >= map.len() as i32 || nj >= map[0].len() as i32 {
            continue;
        }
        if map[ni as usize][nj as usize] != map[i as usize][j as usize] + 1 {
            continue;
        }
        dfs2(map, ni as usize, nj as usize, curr);
    }
}

fn main() {
    // Part 1
    let input = std::fs::read_to_string("input.txt").unwrap();

    let map = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut ans = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                dfs1(&map, i, j, &mut ans, &mut HashSet::new());
            }
        }
    }
    println!("{}", ans);

    // Part 2
    let input = std::fs::read_to_string("input.txt").unwrap();

    let map = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut ans = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                dfs2(&map, i, j, &mut ans);
            }
        }
    }
    println!("{}", ans);
}
