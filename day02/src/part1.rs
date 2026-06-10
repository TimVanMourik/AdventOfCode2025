use crate::IdRange;

fn check_valid(item: i64) -> bool {
    let i_string = item.to_string();
    let i_length = i_string.len();
    if i_length % 2 == 1 {
        return true;
    }

    if i_string[..i_length / 2] == i_string[i_length / 2..] {
        return false;
    }

    true
}

fn compute_valid(item: &IdRange) -> Vec<i64> {
    let first_str = item.first.to_string();
    let last_str = item.last.to_string();

    let mut invalid_ids: Vec<i64> = Vec::new();
    if first_str.len() == last_str.len() && first_str.len() % 2 == 1 {
        return invalid_ids;
    }

    for i in item.first..=item.last {
        if !check_valid(i) {
            invalid_ids.push(i);
        }
    }
    invalid_ids
}

pub fn part1(items: &[IdRange]) -> i64 {
    items
        .iter()
        .fold(Vec::new(), |mut acc, range| {
            acc.extend(compute_valid(range));
            acc
        })
        .iter()
        .sum()
}
