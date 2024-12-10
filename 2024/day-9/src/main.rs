#[derive(Debug, PartialEq)]
enum Byte {
    BlockId(i128),
    Empty(i128),
}

impl Byte {
    fn is_empty(&self) -> bool {
        matches!(self, Byte::Empty(_))
    }
}

fn get_next_gap(result: &[Byte], left: usize) -> (usize, usize) {
    let mut ll = left;

    while ll < result.len() && !result[ll].is_empty() {
        ll += 1;
    }

    if ll == result.len() {
        return (ll, ll);
    }

    let mut lr = ll;
    while lr < result.len() && result[lr] == result[ll] {
        lr += 1;
    }
    lr -= 1;

    (ll, lr)
}

fn get_next_file(result: &[Byte], right: usize) -> (usize, usize) {
    let mut rr = right;

    while rr > 0 && result[rr].is_empty() {
        rr -= 1;
    }
    let mut rl = rr;
    while rl > 0 && result[rl] == result[rr] {
        rl -= 1;
    }
    rl += 1;

    (rl, rr)
}

fn main() {
    // Part 1
    let input = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let mut result = Vec::new();
    for i in (0..input.len()).step_by(2) {
        for _ in 0..input[i] {
            result.push(i as i128 / 2);
        }

        if i + 1 == input.len() {
            break;
        }

        for _ in 0..input[i + 1] {
            result.push(-1);
        }
    }

    let (mut left, mut right) = (0, result.len() - 1);
    while left < right {
        while right > 0 && result[right] == -1 {
            right -= 1;
        }

        while left < result.len() && result[left] != -1 {
            left += 1;
        }

        if left >= right {
            break;
        }

        result.swap(left, right);
    }

    let mut ans = 0;
    for (i, block_id) in result.into_iter().enumerate() {
        if block_id == -1 {
            break;
        }

        ans += i as i128 * block_id;
    }
    println!("{:?}", ans);

    // Part 2
    let input = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let mut result = Vec::new();
    for i in (0..input.len()).step_by(2) {
        for _ in 0..input[i] {
            result.push(Byte::BlockId(i as i128 / 2));
        }

        if i + 1 == input.len() {
            break;
        }

        for _ in 0..input[i + 1] {
            result.push(Byte::Empty(i as i128 / 2));
        }
    }

    let (mut left, mut right) = (0, result.len() - 1);

    while left < right {
        let (mut next_ll, mut next_lr) = get_next_gap(&result, 0);
        let (next_rl, next_rr) = get_next_file(&result, right);

        while next_ll < result.len() && next_lr - next_ll < next_rr - next_rl {
            (next_ll, next_lr) = get_next_gap(&result, next_ll + 1);
        }

        if next_lr < next_rl && next_lr - next_ll >= next_rr - next_rl {
            for i in next_rl..=next_rr {
                result.swap(next_ll, i);
                next_ll += 1;
            }
            left = 0;
            right = result.len() - 1;
        } else {
            if next_ll == 0 {
                break;
            }
            right = next_rl - 1;
        }
    }

    let mut ans = 0;
    for (i, block_id) in result.into_iter().enumerate() {
        if let Byte::BlockId(block_id) = block_id {
            ans += i as i128 * block_id;
        }
    }
    println!("{:?}", ans);
}
