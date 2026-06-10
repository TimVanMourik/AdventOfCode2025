use crate::Warehouse;

const MAX_NEIGHBOURS: i64 = 3;

pub fn part1(warehouse: &Warehouse) -> i64 {
    warehouse.shelves.iter().fold(0, |acc, row| {
        acc + row.iter().fold(0, |row_acc, shelf| {
            row_acc
                + if !shelf.has_paper {
                    0
                } else {
                    let neighbour_count = warehouse
                        .neighbours(shelf.y, shelf.x)
                        .filter(|n| n.has_paper)
                        .count() as i64;
                    if neighbour_count > MAX_NEIGHBOURS {
                        0
                    } else {
                        1
                    }
                }
        })
    })
}
