use std::fs;

fn is_solvable_part_1(test_value: i128, values: &[i128], i: usize, curr: i128) -> bool {
    if i == values.len() {
        return curr == test_value;
    }

    if is_solvable_part_1(test_value, values, i + 1, curr + values[i]) {
        return true;
    }

    if is_solvable_part_1(test_value, values, i + 1, curr * values[i]) {
        return true;
    }

    false
}

fn is_solvable_part_2(test_value: i128, values: &[i128], i: usize, curr: i128) -> bool {
    if i == values.len() {
        return curr == test_value;
    }

    if is_solvable_part_2(test_value, values, i + 1, curr + values[i]) {
        return true;
    }

    if is_solvable_part_2(test_value, values, i + 1, curr * values[i]) {
        return true;
    }

    // Part 2 condition
    if is_solvable_part_2(
        test_value,
        values,
        i + 1,
        format!("{}{}", curr, values[i]).parse().unwrap(),
    ) {
        return true;
    }

    false
}

fn main() {
    // Part 1
    let input = fs::read_to_string("input.txt").unwrap();

    let mut ans = 0;
    for line in input.lines() {
        let parsed = line.split(':').collect::<Vec<&str>>();
        let test_value = parsed[0].parse::<i128>().unwrap();
        let values = parsed[1]
            .split_ascii_whitespace()
            .map(|x| x.parse::<i128>().unwrap())
            .collect::<Vec<_>>();

        if is_solvable_part_1(test_value, &values, 0, 0) {
            ans += test_value;
        }
    }

    println!("{:?}", ans);

    // Part 2
    let input = fs::read_to_string("input.txt").unwrap();

    let mut ans = 0;
    for line in input.lines() {
        let parsed = line.split(':').collect::<Vec<&str>>();
        let test_value = parsed[0].parse::<i128>().unwrap();
        let values = parsed[1]
            .split_ascii_whitespace()
            .map(|x| x.parse::<i128>().unwrap())
            .collect::<Vec<_>>();

        if is_solvable_part_2(test_value, &values, 0, 0) {
            ans += test_value;
        }
    }

    println!("{:?}", ans);
}
