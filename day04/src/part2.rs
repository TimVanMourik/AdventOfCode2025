use crate::Warehouse;

pub fn part2(warehouse: &Warehouse) -> i64 {
    let mut warehouse = warehouse.clone();
    let mut count = 0;

    loop {
        let rolls = warehouse.removable_rolls();
        if rolls.is_empty() {
            break;
        }

        count += rolls.len() as i64;
        warehouse.remove_rolls(&rolls);
    }

    count
}
