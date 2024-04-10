// https://leetcode.com/problems/reveal-cards-in-increasing-order/

pub struct Solution;

// Initial attempt (does not implement the correct algorithm)
// impl Solution {
//     pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
//         deck.sort_unstable();
//         let (faceups, facedowns) = deck.split_at((deck.len() + 1) / 2);
//         let mut result = std::vec::Vec::with_capacity(deck.len());
//         let data = result.spare_capacity_mut();
//         for i in 0..faceups.len() {
//             data[2 * i].write(faceups[i]);
//             if i < facedowns.len() {
//                 data[2 * i + 1].write(facedowns[i]);
//             }
//         }
//         unsafe {
//             result.set_len(deck.len());
//         }
//         result
//     }
// }

impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        deck.sort_unstable();
        let mut result = std::collections::vec_deque::VecDeque::with_capacity(deck.len());
        while let Some(card) = deck.pop() {
            if result.len() > 0 {
                result.rotate_right(1);
            }
            result.push_front(card);
        }
        result.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let deck = vec![17, 13, 11, 2, 3, 5, 7];
        let result = Solution::deck_revealed_increasing(deck);
        assert_eq!(result, vec![2, 13, 3, 11, 5, 17, 7])
    }

    #[test]
    fn ex2() {
        let deck = vec![1, 1000];
        let result = Solution::deck_revealed_increasing(deck);
        assert_eq!(result, vec![1, 1000]);
    }
}
