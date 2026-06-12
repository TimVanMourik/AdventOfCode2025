use crate::{FreshRange, Ingredients};

fn accumulate_range((count, index): (i64, i64), range: &FreshRange) -> (i64, i64) {
    if range.end < index {
        // Range is completely contained in the previous range, don't add anything
        (count, index)
    } else if range.start < index {
        // Range partially overlaps, only count the new part
        (count + range.end - index + 1, range.end + 1)
    } else {
        // No overlap, count the entire range
        (count + range.end - range.start + 1, range.end + 1)
    }
}

pub fn part2(items: &Ingredients) -> i64 {
    // let mut count = 0;
    let mut ids = items.fresh_ranges.clone();
    ids.sort_by(|a, b| a.start.cmp(&b.start));
    ids.iter().fold((0i64, 0i64), accumulate_range).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accumulate_range_when_start_less_than_index() {
        // When range.start (5) < index (10)
        let range = FreshRange { start: 5, end: 15 };
        let (count, index) = (100, 10);

        let (new_count, new_index) = accumulate_range((count, index), &range);

        // count + range.end - index + 1 = 100 + 15 - 10 + 1 = 106
        assert_eq!(new_count, 106);
        // range.end + 1 = 15 + 1 = 16
        assert_eq!(new_index, 16);
    }

    #[test]
    fn test_accumulate_range_when_start_equals_index() {
        // When range.start (10) >= index (10)
        let range = FreshRange { start: 10, end: 20 };
        let (count, index) = (50, 10);

        let (new_count, new_index) = accumulate_range((count, index), &range);

        // count + range.end - range.start + 1 = 50 + 20 - 10 + 1 = 61
        assert_eq!(new_count, 61);
        // range.end + 1 = 20 + 1 = 21
        assert_eq!(new_index, 21);
    }

    #[test]
    fn test_accumulate_range_when_start_greater_than_index() {
        // When range.start (20) > index (10)
        let range = FreshRange { start: 20, end: 30 };
        let (count, index) = (0, 10);

        let (new_count, new_index) = accumulate_range((count, index), &range);

        // count + range.end - range.start + 1 = 0 + 30 - 20 + 1 = 11
        assert_eq!(new_count, 11);
        // range.end + 1 = 30 + 1 = 31
        assert_eq!(new_index, 31);
    }

    #[test]
    fn test_accumulate_range_with_single_element_range() {
        // Test with a range that contains only one element
        let range = FreshRange { start: 5, end: 5 };
        let (count, index) = (100, 3);

        let (new_count, new_index) = accumulate_range((count, index), &range);

        // count + range.end - range.start + 1 = 100 + 5 - 5 + 1 = 101
        assert_eq!(new_count, 101);
        assert_eq!(new_index, 6);
    }

    #[test]
    fn test_accumulate_range_with_zero_count() {
        // Test starting with zero count
        let range = FreshRange { start: 0, end: 10 };
        let (count, index) = (0, 5);

        let (new_count, new_index) = accumulate_range((count, index), &range);

        // count + range.end - index + 1 = 0 + 10 - 5 + 1 = 6
        assert_eq!(new_count, 6);
        assert_eq!(new_index, 11);
    }

    #[test]
    fn test_accumulate_range_with_large_values() {
        // Test with large i64 values
        let range = FreshRange {
            start: 1000000,
            end: 2000000,
        };
        let (count, index) = (5000000, 1500000);

        let (new_count, new_index) = accumulate_range((count, index), &range);

        // count + range.end - index + 1 = 5000000 + 2000000 - 1500000 + 1 = 5500001
        assert_eq!(new_count, 5500001);
        assert_eq!(new_index, 2000001);
    }

    #[test]
    fn test_accumulate_range_completely_contained() {
        // Test when range is completely contained within previous range
        let range = FreshRange { start: 5, end: 8 };
        let (count, index) = (100, 15); // Previous range ended at 14, we're at 15

        let (new_count, new_index) = accumulate_range((count, index), &range);

        // Range (5, 8) is completely before index 15, so don't add anything
        assert_eq!(new_count, 100);
        assert_eq!(new_index, 15); // Index should not change
    }
}
