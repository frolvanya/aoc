use std::collections::HashSet;

fn dfs(
    grid: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    group: char,
    visited: &mut Vec<Vec<bool>>,
    path: &mut HashSet<(usize, usize)>,
) {
    path.insert((i, j));
    visited[i][j] = true;

    for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let x = i as i32 + dx;
        let y = j as i32 + dy;
        if x < 0 || y < 0 || x >= grid.len() as i32 || y >= grid[0].len() as i32 {
            continue;
        }
        let x = x as usize;
        let y = y as usize;
        if visited[x][y] {
            continue;
        }
        if grid[x][y] == group {
            dfs(grid, x, y, group, visited, path);
        }
    }
}

fn perimeter(grid: &[Vec<char>], path: &HashSet<(usize, usize)>) -> usize {
    let mut perimeter = 0;

    for (x, y) in path {
        let x = *x;
        let y = *y;

        if x == 0 || grid[x - 1][y] != grid[x][y] {
            perimeter += 1;
        }
        if y == 0 || grid[x][y - 1] != grid[x][y] {
            perimeter += 1;
        }
        if x == grid.len() - 1 || grid[x + 1][y] != grid[x][y] {
            perimeter += 1;
        }
        if y == grid[0].len() - 1 || grid[x][y + 1] != grid[x][y] {
            perimeter += 1;
        }
    }

    perimeter
}

fn sides(path: &HashSet<(usize, usize)>) -> usize {
    let mut total_sides = 0;

    for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let mut sides = HashSet::new();

        for &(x, y) in path {
            let neighbor = ((x as i32 + dir.0) as usize, (y as i32 + dir.1) as usize);
            if !path.contains(&neighbor) {
                sides.insert(neighbor);
            }
        }

        let mut remove = HashSet::new();
        for &side in &sides {
            let mut tmp = (
                (side.0 as i32 + dir.1) as usize,
                (side.1 as i32 + dir.0) as usize,
            );
            while sides.contains(&tmp) {
                remove.insert(tmp);
                tmp = (
                    (tmp.0 as i32 + dir.1) as usize,
                    (tmp.1 as i32 + dir.0) as usize,
                );
            }
        }

        total_sides += sides.len() - remove.len();
    }

    total_sides
}

fn main() {
    // Part 1
    let input = include_str!("../input.txt");
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if visited[i][j] {
                continue;
            }

            let mut path = HashSet::new();
            dfs(&grid, i, j, grid[i][j], &mut visited, &mut path);
            ans += path.len() * perimeter(&grid, &path);
        }
    }
    println!("{}", ans);

    // Part 2
    let input = include_str!("../input.txt");
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if visited[i][j] {
                continue;
            }

            let mut path = HashSet::new();
            dfs(&grid, i, j, grid[i][j], &mut visited, &mut path);
            ans += path.len() * sides(&path);
        }
    }
    println!("{}", ans);
}
