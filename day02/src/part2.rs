use crate::IdRange;

fn slices_for(n: usize) -> &'static [usize] {
    match n {
        1 => &[],
        2 | 3 => &[1],
        4 => &[1, 2],
        5 => &[1],
        6 => &[1, 2, 3],
        7 => &[1],
        8 => &[1, 2, 4],
        9 => &[1, 3],
        10 => &[1, 2, 5],
        _ => &[],
    }
}

fn chop_string(item: i64, n: &usize) -> Vec<String> {
    let i_string = item.to_string();
    let i_length = i_string.len();
    if *n == 0 || *n > i_length {
        panic!("invalid n: {}", n);
    }
    let mut result: Vec<String> = Vec::new();
    for i in 0..i_length / *n {
        result.push(i_string[i * n..(i + 1) * n].to_string());
    }
    result
}

fn check_valid(item: i64) -> bool {
    for n in slices_for(item.to_string().len()) {
        let slices = chop_string(item, n);
        if let Some(first) = slices.first() {
            if slices.iter().all(|s| s == first) {
                return false;
            }
        }
    }

    true
}

fn compute_valid(item: &IdRange) -> Vec<i64> {
    let mut invalid_ids: Vec<i64> = Vec::new();
    for i in item.first..=item.last {
        if !check_valid(i) {
            invalid_ids.push(i);
        }
    }
    invalid_ids
}

pub fn part2(items: &[IdRange]) -> i64 {
    items
        .iter()
        .fold(Vec::new(), |mut acc, range| {
            acc.extend(compute_valid(range));
            acc
        })
        .iter()
        .sum()
}
