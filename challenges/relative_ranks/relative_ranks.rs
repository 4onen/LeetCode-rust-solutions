// https://leetcode.com/problems/relative-ranks/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
//         let mut sorted_score = score
//             .iter()
//             .copied()
//             .collect::<std::collections::BinaryHeap<_>>();
//         let mut places: std::collections::HashMap<i32, u16> = std::collections::HashMap::new();
//         let mut place = unsafe { std::num::NonZeroU16::new_unchecked(1) };
//         while let Some(score) = sorted_score.pop() {
//             places.insert(score, place.get());
//             place = place.checked_add(1).unwrap();
//         }
//         score
//             .into_iter()
//             .map(|score| match places[&score] {
//                 1 => "Gold Medal".to_string(),
//                 2 => "Silver Medal".to_string(),
//                 3 => "Bronze Medal".to_string(),
//                 place => place.to_string(),
//             })
//             .collect()
//     }
// }

// One array copy sol'n
impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        assert!(score.len() > 0);
        assert!(score.len() <= 10000);
        let sorted_score = {
            let mut sorted_score = score.clone();
            sorted_score.sort_unstable();
            sorted_score
        };
        let mut ret = Vec::with_capacity(score.len());
        for score in score {
            match sorted_score.len() - sorted_score.binary_search(&score).unwrap() {
                0 => unreachable!("score not found in sorted_score"),
                1 => ret.push("Gold Medal".to_string()),
                2 => ret.push("Silver Medal".to_string()),
                3 => ret.push("Bronze Medal".to_string()),
                place => ret.push(place.to_string()),
            }
        }
        ret
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
