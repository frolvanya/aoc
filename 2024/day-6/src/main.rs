use std::{collections::HashSet, fs};

fn main() {
    // Part 1
    let input = fs::read_to_string("input.txt").unwrap();

    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut x = 0;
    let mut y = 0;

    for (i, row) in map.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if col == &'^' {
                x = i;
                y = j;
                break;
            }
        }
    }

    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    visited[x][y] = true;

    let mut dir = 0;
    loop {
        let (di, dj) = match dir {
            0 => (-1, 0),
            1 => (0, 1),
            2 => (1, 0),
            3 => (0, -1),
            _ => unreachable!(),
        };

        let ni = x as i32 + di;
        let nj = y as i32 + dj;

        if ni < 0 || nj < 0 || ni >= map.len() as i32 || nj >= map[0].len() as i32 {
            break;
        }

        if map[ni as usize][nj as usize] == '#' {
            dir = (dir + 1) % 4;
            continue;
        }

        x = ni as usize;
        y = nj as usize;
        visited[x][y] = true;
    }

    println!(
        "{}",
        visited
            .iter()
            .map(|row| row.iter().filter(|&&v| v).count())
            .sum::<usize>()
    );

    // Part 2
    let input = fs::read_to_string("input.txt").unwrap();

    let mut map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start_x = 0;
    let mut start_y = 0;

    for (i, row) in map.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if col == &'^' {
                start_x = i;
                start_y = j;
                break;
            }
        }
    }

    let mut ans = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != '.' {
                continue;
            }

            map[i][j] = '#';

            let mut x = start_x;
            let mut y = start_y;

            let mut is_loop = false;
            let mut dir = 0;

            let mut was = HashSet::new();
            was.insert((x, y, dir));

            loop {
                let (di, dj) = match dir {
                    0 => (-1, 0),
                    1 => (0, 1),
                    2 => (1, 0),
                    3 => (0, -1),
                    _ => unreachable!(),
                };

                let ni = x as i32 + di;
                let nj = y as i32 + dj;

                if ni < 0 || nj < 0 || ni >= map.len() as i32 || nj >= map[0].len() as i32 {
                    break;
                }

                if map[ni as usize][nj as usize] == '#' {
                    dir = (dir + 1) % 4;
                    continue;
                }

                x = ni as usize;
                y = nj as usize;

                if !was.insert((x, y, dir)) {
                    is_loop = true;
                    break;
                }
            }

            if is_loop {
                ans += 1;
            }

            map[i][j] = '.';
        }
    }

    println!("{}", ans);
}
