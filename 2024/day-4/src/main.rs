use std::fs;

fn is_word_present(
    grid: &[Vec<char>],
    start_row: isize,
    start_col: isize,
    dx: isize,
    dy: isize,
) -> bool {
    let mut row = start_row;
    let mut col = start_col;
    for ch in "XMAS".chars() {
        if row < 0
            || col < 0
            || row >= grid.len() as isize
            || col >= grid[row as usize].len() as isize
        {
            return false;
        }
        if grid[row as usize][col as usize] != ch {
            return false;
        }
        row += dx;
        col += dy;
    }
    true
}

fn x_mas(grid: &[Vec<char>], row: usize, col: usize) -> bool {
    if row + 2 >= grid.len() || col + 2 >= grid[row].len() {
        return false;
    }

    let result = [
        grid[row][col],
        grid[row][col + 2],
        grid[row + 1][col + 1],
        grid[row + 2][col],
        grid[row + 2][col + 2],
    ]
    .iter()
    .collect::<String>();

    ["MSAMS", "SSAMM", "MMASS", "SMASM"].contains(&result.as_str())
}

fn main() {
    // Part 1
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let directions = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let mut ans = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            for &(dx, dy) in &directions {
                if is_word_present(&grid, row as isize, col as isize, dx, dy) {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);

    // Part 2
    let input = fs::read_to_string("example.txt").expect("Failed to read input file");
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut ans = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if x_mas(&grid, row, col) {
                ans += 1
            }
        }
    }

    println!("{}", ans);
}
