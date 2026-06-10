use crate::{Bank, Battery};

struct BatteryInBank {
    value: Battery,
    position: usize,
    index: usize,
}

type Index = usize;

const N_BATTERIES: usize = 12;

fn init_batteries() -> Vec<BatteryInBank> {
    (0..N_BATTERIES)
        .map(|index| BatteryInBank {
            value: 0,
            position: index,
            index: 0,
        })
        .collect()
}

fn find_position_in_bank(bank: &[Battery]) -> (Battery, Index) {
    let mut highest = 0;
    let mut highest_index = 0;
    for (index, &v) in bank.iter().enumerate() {
        if v > highest {
            highest = v;
            highest_index = index;
        }
        if highest == 9 {
            break;
        }
    }

    (highest, highest_index)
}

fn compute_joltage(item: &Bank) -> u64 {
    // init batteries
    let mut batteries = init_batteries();
    for index in 0..batteries.len() {
        let start = if index == 0 {
            0
        } else {
            batteries[index - 1].index + 1
        };
        let end = item.batteries.len() - (N_BATTERIES - batteries[index].position - 1);
        if let Some(slice) = item.batteries.get(start..end) {
            let (value, position) = find_position_in_bank(slice);
            batteries[index].value = value;
            batteries[index].index = start + position;
        } else {
            panic!(
                "Logic error: start index {} is out of bounds for bank with length {}",
                start,
                item.batteries.len()
            );
        }
    }

    batteries
        .iter()
        .fold(0, |acc, battery| acc * 10 + battery.value as u64)
}

pub fn part2(items: &[Bank]) -> u64 {
    items.iter().map(compute_joltage).sum()
}
