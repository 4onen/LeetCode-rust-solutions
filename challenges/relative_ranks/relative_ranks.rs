// https://leetcode.com/problems/relative-ranks/

pub struct Solution;

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut sorted_score = score
            .iter()
            .copied()
            .collect::<std::collections::BinaryHeap<_>>();
        let mut places: std::collections::HashMap<i32, u16> = std::collections::HashMap::new();
        let mut place = unsafe { std::num::NonZeroU16::new_unchecked(1) };
        while let Some(score) = sorted_score.pop() {
            places.insert(score, place.get());
            place = place.checked_add(1).unwrap();
        }
        score
            .into_iter()
            .map(|score| match places[&score] {
                1 => "Gold Medal".to_string(),
                2 => "Silver Medal".to_string(),
                3 => "Bronze Medal".to_string(),
                place => place.to_string(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &[i32], expected: &[&str]) {
        assert_eq!(Solution::find_relative_ranks(input.to_vec()), expected)
    }

    #[test]
    fn ex1() {
        test(
            &[5, 4, 3, 2, 1],
            &["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"],
        )
    }

    #[test]
    fn ex2() {
        test(
            &[10, 3, 8, 9, 4],
            &["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"],
        )
    }
}
