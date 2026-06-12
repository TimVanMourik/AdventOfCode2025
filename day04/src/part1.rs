use crate::Warehouse;

pub fn part1(warehouse: &Warehouse) -> i64 {
    warehouse.removable_rolls().len() as i64
}
