use crate::Bank;

fn compute_joltage(item: &Bank) -> u64 {
    let mut bat_1 = 0;
    let mut bat_2 = 0;

    for (index, &v) in item.batteries.iter().enumerate() {
        // check whether v is greater than the current highest battery (bat_1) or the second highest battery (bat_2)
        if index < item.batteries.len() - 1 && v > bat_1 {
            bat_1 = v;
            bat_2 = 0;
            continue;
        }
        if v > bat_2 {
            bat_2 = v;
            continue;
        }
    }

    (bat_1 as u64) * 10 + (bat_2 as u64)
}

pub fn part1(items: &[Bank]) -> u64 {
    items.iter().map(compute_joltage).sum()
}
