fn find_min_tokens(a: (i64, i64), b: (i64, i64), prize: (i64, i64)) -> Option<i64> {
    let (a_x, a_y) = a;
    let (b_x, b_y) = b;
    let (x_target, y_target) = prize;

    let mut min_cost = None;
    let mut best_solution = None;

    for a in 0..=x_target / a_x {
        let remaining_x = x_target - a * a_x;
        let remaining_y = y_target - a * a_y;

        if remaining_x % b_x == 0 && remaining_y % b_y == 0 {
            let b = remaining_x / b_x;
            if b >= 0 && remaining_y / b_y == b {
                let cost = 3 * a + b;

                if min_cost.is_none() || cost < min_cost.unwrap() {
                    min_cost = Some(cost);
                    best_solution = Some(cost);
                }
            }
        }
    }

    best_solution
}

fn main() {
    // Part 1
    let input = include_str!("../input.txt");

    let lines: Vec<&str> = input.lines().collect();
    let mut ans = 0;
    for i in (0..lines.len()).step_by(4) {
        let a = lines[i]
            .split(": X+")
            .last()
            .unwrap()
            .split(", Y+")
            .map(|coord| coord.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let b = lines[i + 1]
            .split(": X+")
            .last()
            .unwrap()
            .split(", Y+")
            .map(|coord| coord.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let prize = lines[i + 2]
            .split(": X=")
            .last()
            .unwrap()
            .split(", Y=")
            .map(|coord| coord.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        if let Some(solution) = find_min_tokens((a[0], a[1]), (b[0], b[1]), (prize[0], prize[1])) {
            ans += solution;
        }
    }
    println!("{:?}", ans);
}
