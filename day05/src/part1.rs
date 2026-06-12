use crate::Ingredients;

pub fn part1(items: &Ingredients) -> i64 {
    items.ids.iter().filter(|id| items.is_in_range(id)).count() as i64
}
